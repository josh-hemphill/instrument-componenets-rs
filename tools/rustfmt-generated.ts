/** Run rustfmt on generated Rust files so codegen matches `cargo fmt --check`. */
export async function rustfmtGenerated(urls: URL[]): Promise<void> {
  const args = urls.map((u) => fileUrlToPath(u));
  try {
    const { code, stderr } = await new Deno.Command("rustfmt", {
      args,
      stdout: "piped",
      stderr: "piped",
    }).output();
    if (code !== 0) {
      console.warn(
        "rustfmt failed:",
        new TextDecoder().decode(stderr).trim() || `exit ${code}`,
      );
    }
  } catch (error) {
    console.warn(
      "rustfmt not available; run `cargo fmt --all` after codegen.",
      error instanceof Error ? error.message : error,
    );
  }
}

function fileUrlToPath(url: URL): string {
  let path = url.pathname;
  if (Deno.build.os === "windows" && path.startsWith("/")) {
    path = path.slice(1);
  }
  return path;
}
