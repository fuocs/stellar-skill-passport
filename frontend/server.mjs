import { createReadStream, existsSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, extname, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));
const port = Number.parseInt(process.env.PORT ?? "5173", 10);
const rootPrefix = root.endsWith(sep) ? root : `${root}${sep}`;

const types = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
};

createServer((request, response) => {
  const pathname = new URL(request.url ?? "/", "http://localhost").pathname;
  let requestedPath = "index.html";

  try {
    requestedPath = pathname === "/" ? "index.html" : `.${decodeURIComponent(pathname)}`;
  } catch {
    response.writeHead(400, { "Content-Type": "text/plain; charset=utf-8" });
    response.end("Bad request");
    return;
  }

  const filePath = resolve(root, requestedPath);

  if ((!filePath.startsWith(rootPrefix) && filePath !== root) || !existsSync(filePath)) {
    response.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    response.end("Not found");
    return;
  }

  response.writeHead(200, { "Content-Type": types[extname(filePath)] ?? "application/octet-stream" });
  createReadStream(filePath).pipe(response);
}).listen(port, "127.0.0.1", () => {
  console.log(`Skill Passport frontend: http://127.0.0.1:${port}/`);
});
