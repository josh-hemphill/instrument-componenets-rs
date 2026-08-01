# Capability matrix (IVI-inspired, not IVI-conformant)

Rows are **Base** (target before calling a class “release-ready”) or **Extension** (optional). Status: `done` / `partial` / `todo` / `deferred`.

Copied from the agent-oriented source of truth in `docs/capability-matrix.md`.

## Dmm

| Group | Capability | Status |
|---|---|---|
| Base | Measure Vdc / Vac / Idc / Ω | partial |
| Base | Configure range + resolution | partial |
| Base | Immediate / software trigger + Read / Initiate+Fetch | todo |
| Base | AC current | todo |
| Base | 2-wire / 4-wire resistance | todo |
| Extension | Temperature | todo |
| Extension | Multipoint / sample count | deferred |

## DcPowerSupply

| Group | Capability | Status |
|---|---|---|
| Base | Set V / I limit, enable output, readback | done |
| Base | Query output state | todo |
| Base | OVP set / enable / query | todo |
| Base | Channel count helper | todo |
| Extension | Remote sense | todo |

## FunctionGenerator

| Group | Capability | Status |
|---|---|---|
| Base | Standard waveform, frequency, amplitude, offset, output | done |
| Base | Duty cycle, output load | todo |
| Base | Burst + trigger (imm/ext/bus) | todo |
| Extension | Arbitrary waveform | deferred |

## Oscilloscope

| Group | Capability | Status |
|---|---|---|
| Base | Timebase, channel scale, run/stop, ASCII trace | partial |
| Base | Channel enable + coupling | todo |
| Base | Edge trigger | todo |
| Base | Single acquire | todo |
| Base | Vpp / frequency measurement | todo |
| Extension | Binary waveform | todo |

## Switch

| Group | Capability | Status |
|---|---|---|
| Base | Close / open / is_closed / open_all (matrix pair) | done |
| Base | Path naming helpers | todo |
| Extension | Scan list | deferred |

## Counter

| Group | Capability | Status |
|---|---|---|
| Base | Frequency / period / totalize | done |
| Base | Gate time, channel select, timeout | todo |

## PowerMeter

| Group | Capability | Status |
|---|---|---|
| Base | Configure units / auto-range / avg / corr freq / offset | todo |
| Base | Read / Initiate+Fetch | todo |
| Extension | Dual channel, manual range, cal, ref osc | deferred |

## SpectrumAnalyzer

| Group | Capability | Status |
|---|---|---|
| Base | Center, span, RBW/VBW, ref level | todo |
| Base | Trace fetch (ASCII), marker peak | todo |
| Base | Single / continuous sweep + wait | todo |
| Extension | ACLR / OBW / EMI detectors | deferred |
