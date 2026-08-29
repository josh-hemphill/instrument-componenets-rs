use instrument_core::classifier::{
    classify_from_address, classify_from_identity, merge_classifications, ClassifiedKind,
    ClassifySource,
};
use instrument_core::dialect::{resolve_dialect, DIALECT_PROFILES};
use instrument_core::identity::Idn;
use instrument_core::kind::InstrumentKind;
use instrument_core::registry::ModelRegistry;
use instrument_core::ResourceAddress;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

fn spec_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../spec")
        .join(name)
}

fn load_json<T: for<'de> Deserialize<'de>>(name: &str) -> T {
    let path = spec_path(name);
    let json =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&json).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScpiVectors {
    generic_templates: Vec<GenericTemplate>,
    dialect_resolve: Vec<DialectResolve>,
    dialect_commands: Vec<DialectCommand>,
    formatted_commands: Vec<FormattedCommand>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenericTemplate {
    profile_id: String,
    key: String,
    command: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DialectResolve {
    id: String,
    kind: String,
    manufacturer: Option<String>,
    model: Option<String>,
    expected_profile: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DialectCommand {
    id: String,
    kind: String,
    manufacturer: Option<String>,
    model: Option<String>,
    key: String,
    command: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormattedCommand {
    id: String,
    kind: String,
    manufacturer: Option<String>,
    model: Option<String>,
    key: String,
    vars: HashMap<String, String>,
    command: String,
}

#[derive(Debug, Deserialize)]
struct ClassifierFile {
    cases: Vec<ClassifierCase>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClassifierCase {
    id: String,
    layer: String,
    address: Option<String>,
    idn: Option<IdnDto>,
    expected_identity: Option<IdentityDto>,
    expected_kinds: Option<Vec<KindDto>>,
    base_layers: Option<Vec<Vec<KindDto>>>,
    override_kinds: Option<Vec<String>>,
    expected_supported: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct IdnDto {
    manufacturer: String,
    model: String,
    serial: String,
    firmware: String,
}

#[derive(Debug, Deserialize)]
struct IdentityDto {
    manufacturer: Option<String>,
    model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct KindDto {
    kind: String,
    confidence: u8,
    source: String,
}

fn parse_kind(label: &str) -> InstrumentKind {
    InstrumentKind::from_str_label(label).unwrap_or_else(|| panic!("unknown kind {label}"))
}

fn parse_source(label: &str) -> ClassifySource {
    match label {
        "ResourceParse" => ClassifySource::ResourceParse,
        "VisaAttributes" => ClassifySource::VisaAttributes,
        "ModelRegistry" => ClassifySource::ModelRegistry,
        "ScpiIdn" => ClassifySource::ScpiIdn,
        "CapabilityProbe" => ClassifySource::CapabilityProbe,
        "UserOverride" => ClassifySource::UserOverride,
        other => panic!("unknown classify source {other}"),
    }
}

fn profile_by_id(id: &str) -> &'static instrument_core::DialectProfile {
    DIALECT_PROFILES
        .iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("unknown profile {id}"))
}

fn resolve_case(
    kind: &str,
    manufacturer: Option<&str>,
    model: Option<&str>,
) -> &'static instrument_core::DialectProfile {
    resolve_dialect(parse_kind(kind), manufacturer, model)
}

fn kind_tuples(kinds: &[ClassifiedKind]) -> Vec<(InstrumentKind, u8, ClassifySource)> {
    let mut rows: Vec<_> = kinds
        .iter()
        .map(|k| (k.kind, k.confidence, k.source))
        .collect();
    rows.sort_by_key(|k| format!("{:?}:{:?}:{}", k.0, k.2, k.1));
    rows
}

fn expected_kind_tuples(kinds: &[KindDto]) -> Vec<(InstrumentKind, u8, ClassifySource)> {
    let mut rows: Vec<_> = kinds
        .iter()
        .map(|k| (parse_kind(&k.kind), k.confidence, parse_source(&k.source)))
        .collect();
    rows.sort_by_key(|k| format!("{:?}:{:?}:{}", k.0, k.2, k.1));
    rows
}

#[test]
fn generic_templates_match_profiles() {
    let vectors: ScpiVectors = load_json("scpi-vectors.json");
    for row in vectors.generic_templates {
        let actual = profile_by_id(&row.profile_id)
            .command(&row.key)
            .unwrap_or_else(|| panic!("{} missing key {}", row.profile_id, row.key));
        assert_eq!(
            actual, row.command,
            "generic template {}.{}",
            row.profile_id, row.key
        );
    }
}

#[test]
fn dialect_resolve_matches_vectors() {
    let vectors: ScpiVectors = load_json("scpi-vectors.json");
    for row in vectors.dialect_resolve {
        let profile = resolve_case(&row.kind, row.manufacturer.as_deref(), row.model.as_deref());
        assert_eq!(profile.id, row.expected_profile, "resolve {}", row.id);
    }
}

#[test]
fn dialect_commands_match_vectors() {
    let vectors: ScpiVectors = load_json("scpi-vectors.json");
    for row in vectors.dialect_commands {
        let profile = resolve_case(&row.kind, row.manufacturer.as_deref(), row.model.as_deref());
        let actual = profile
            .command(&row.key)
            .unwrap_or_else(|| panic!("{} missing {}", row.id, row.key));
        assert_eq!(actual, row.command, "command {}", row.id);
    }
}

#[test]
fn formatted_commands_match_vectors() {
    let vectors: ScpiVectors = load_json("scpi-vectors.json");
    for row in vectors.formatted_commands {
        let profile = resolve_case(&row.kind, row.manufacturer.as_deref(), row.model.as_deref());
        let vars: Vec<(String, String)> = row.vars.into_iter().collect();
        let vars_ref: Vec<(&str, String)> =
            vars.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
        let actual = profile
            .format_command(&row.key, &vars_ref)
            .unwrap_or_else(|| panic!("{} missing {}", row.id, row.key));
        assert_eq!(actual, row.command, "formatted {}", row.id);
    }
}

#[test]
fn classifier_cases_match_vectors() {
    let file: ClassifierFile = load_json("classifier-cases.json");
    let registry = ModelRegistry::embedded();
    for case in file.cases {
        match case.layer.as_str() {
            "address" => {
                let addr =
                    ResourceAddress::parse(case.address.as_deref().expect("address")).unwrap();
                let (identity, kinds) = classify_from_address(&addr, &registry);
                if let Some(expected) = case.expected_identity {
                    if let Some(mfr) = expected.manufacturer {
                        assert_eq!(
                            identity.manufacturer.as_deref(),
                            Some(mfr.as_str()),
                            "{}",
                            case.id
                        );
                    }
                    if let Some(model) = expected.model {
                        assert_eq!(
                            identity.model.as_deref(),
                            Some(model.as_str()),
                            "{}",
                            case.id
                        );
                    }
                }
                let expected = case.expected_kinds.expect("expectedKinds");
                assert_eq!(
                    kind_tuples(&kinds),
                    expected_kind_tuples(&expected),
                    "{}",
                    case.id
                );
            }
            "idn" => {
                let idn = case.idn.expect("idn");
                let parsed = Idn {
                    manufacturer: idn.manufacturer,
                    model: idn.model,
                    serial: idn.serial,
                    firmware: idn.firmware,
                };
                let (identity, kinds) = classify_from_identity(&parsed, &registry);
                if let Some(expected) = case.expected_identity {
                    if let Some(mfr) = expected.manufacturer {
                        assert_eq!(
                            identity.manufacturer.as_deref(),
                            Some(mfr.as_str()),
                            "{}",
                            case.id
                        );
                    }
                    if let Some(model) = expected.model {
                        assert_eq!(
                            identity.model.as_deref(),
                            Some(model.as_str()),
                            "{}",
                            case.id
                        );
                    }
                }
                let expected = case.expected_kinds.expect("expectedKinds");
                assert_eq!(
                    kind_tuples(&kinds),
                    expected_kind_tuples(&expected),
                    "{}",
                    case.id
                );
            }
            "override" | "merge" => {
                let layers = case
                    .base_layers
                    .expect("baseLayers")
                    .into_iter()
                    .map(|layer| {
                        layer
                            .into_iter()
                            .map(|k| ClassifiedKind {
                                kind: parse_kind(&k.kind),
                                confidence: k.confidence,
                                source: parse_source(&k.source),
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                let override_kinds: Option<Vec<InstrumentKind>> = case
                    .override_kinds
                    .map(|ks| ks.iter().map(|k| parse_kind(k)).collect());
                let (supported, _) = merge_classifications(layers, override_kinds.as_deref());
                let expected: Vec<InstrumentKind> = case
                    .expected_supported
                    .expect("expectedSupported")
                    .iter()
                    .map(|k| parse_kind(k))
                    .collect();
                assert_eq!(supported, expected, "{}", case.id);
            }
            other => panic!("unknown classifier layer {other} in {}", case.id),
        }
    }
}
