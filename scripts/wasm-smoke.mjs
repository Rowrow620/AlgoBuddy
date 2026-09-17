import assert from "node:assert/strict";
import { createServer } from "node:http";
import { mkdir, readFile, stat } from "node:fs/promises";
import path from "node:path";
import { chromium } from "playwright";

const mountPath = "/AlgoBuddy/";
const siteDirectory = path.resolve(process.env.SMOKE_SITE_DIR ?? "dist");
const resultsDirectory = path.resolve("test-results");
const canonicalScreenshotPath = path.join(resultsDirectory, "wasm-smoke.png");
const liveBaseUrl = process.env.SMOKE_BASE_URL;
let expectedRevision = process.env.EXPECTED_DEPLOY_SHA?.trim();
const mimeTypes = new Map([
  [".css", "text/css; charset=utf-8"],
  [".html", "text/html; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
  [".txt", "text/plain; charset=utf-8"],
  [".wasm", "application/wasm"],
]);

function positiveInteger(value, fallback, name) {
  if (value === undefined) {
    return fallback;
  }

  const parsed = Number.parseInt(value, 10);
  if (!Number.isSafeInteger(parsed) || parsed < 1) {
    throw new Error(`${name} must be a positive integer`);
  }
  return parsed;
}

const liveLaunchAttempts = positiveInteger(process.env.SMOKE_ATTEMPTS, 3, "SMOKE_ATTEMPTS");
const markerAttempts = positiveInteger(
  process.env.SMOKE_MARKER_ATTEMPTS,
  20,
  "SMOKE_MARKER_ATTEMPTS",
);
const retryDelayMs = positiveInteger(
  process.env.SMOKE_RETRY_DELAY_MS,
  3_000,
  "SMOKE_RETRY_DELAY_MS",
);

function normalizeBaseUrl(value) {
  const url = new URL(value);
  if (!url.pathname.endsWith("/")) {
    url.pathname += "/";
  }
  return url;
}

function contentType(filePath) {
  return mimeTypes.get(path.extname(filePath)) ?? "application/octet-stream";
}

function delay(milliseconds) {
  return new Promise(resolve => setTimeout(resolve, milliseconds));
}

async function resolveExpectedRevision() {
  if (!expectedRevision && !liveBaseUrl) {
    expectedRevision = (await readFile(path.join(siteDirectory, "deploy-sha.txt"), "utf8")).trim();
  }
  if (!expectedRevision) {
    throw new Error("EXPECTED_DEPLOY_SHA is required for live smoke tests");
  }
  if (!/^[0-9a-f]{40}$/i.test(expectedRevision)) {
    throw new Error("Expected deployment revision must be a 40-character Git commit SHA");
  }
}

function revisionUrl(baseUrl, relativePath, attempt) {
  const url = new URL(relativePath, baseUrl);
  if (expectedRevision) {
    url.searchParams.set("revision", expectedRevision);
  }
  url.searchParams.set("attempt", String(attempt));
  url.searchParams.set("cache", String(Date.now()));
  return url;
}

async function createStaticServer() {
  const server = createServer(async (request, response) => {
    try {
      const requestUrl = new URL(request.url ?? "/", "http://127.0.0.1");
      if (!requestUrl.pathname.startsWith(mountPath)) {
        response.writeHead(404).end("Not found");
        return;
      }

      const relativePath = decodeURIComponent(requestUrl.pathname.slice(mountPath.length));
      const requestedFile = relativePath.length === 0 ? "index.html" : relativePath;
      const filePath = path.resolve(siteDirectory, requestedFile);
      const sitePrefix = `${siteDirectory}${path.sep}`;
      if (filePath !== path.join(siteDirectory, "index.html") && !filePath.startsWith(sitePrefix)) {
        response.writeHead(403).end("Forbidden");
        return;
      }

      const fileStats = await stat(filePath);
      if (!fileStats.isFile()) {
        response.writeHead(404).end("Not found");
        return;
      }

      const body = await readFile(filePath);
      response.writeHead(200, {
        "cache-control": "no-store",
        "content-length": body.length,
        "content-type": contentType(filePath),
      });
      response.end(request.method === "HEAD" ? undefined : body);
    } catch (error) {
      const statusCode = error?.code === "ENOENT" ? 404 : 500;
      response.writeHead(statusCode).end(statusCode === 404 ? "Not found" : "Server error");
    }
  });

  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });

  const address = server.address();
  assert(address && typeof address !== "string", "Smoke test server did not open a TCP port");
  return {
    baseUrl: new URL(mountPath, `http://127.0.0.1:${address.port}`),
    close: () => new Promise((resolve, reject) => {
      server.close(error => error ? reject(error) : resolve());
    }),
  };
}

async function fetchTextWithTimeout(url, timeoutMs) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(url, {
      cache: "no-store",
      headers: {
        "cache-control": "no-cache",
        pragma: "no-cache",
      },
      redirect: "follow",
      signal: controller.signal,
    });
    return {
      body: await response.text(),
      status: response.status,
    };
  } finally {
    clearTimeout(timeout);
  }
}

async function waitForPublishedRevision(baseUrl) {
  if (!expectedRevision) {
    return;
  }

  let lastResult = "no response";
  for (let attempt = 1; attempt <= markerAttempts; attempt += 1) {
    try {
      const markerUrl = revisionUrl(baseUrl, "deploy-sha.txt", attempt);
      const result = await fetchTextWithTimeout(markerUrl, 5_000);
      const publishedRevision = result.body.trim();
      if (result.status === 200 && publishedRevision === expectedRevision) {
        console.log(`GitHub Pages published revision ${expectedRevision}`);
        return;
      }
      lastResult = `HTTP ${result.status}, revision ${publishedRevision || "missing"}`;
    } catch (error) {
      lastResult = error instanceof Error ? error.message : String(error);
    }

    if (attempt < markerAttempts) {
      console.log(
        `Waiting for revision ${expectedRevision} ` +
        `(attempt ${attempt} of ${markerAttempts}: ${lastResult})`,
      );
      await delay(retryDelayMs);
    }
  }

  throw new Error(
    `GitHub Pages did not publish revision ${expectedRevision}: ${lastResult}`,
  );
}

async function saveFailureScreenshots(page, mode, attempt) {
  await mkdir(resultsDirectory, { recursive: true });
  const attemptPath = path.join(resultsDirectory, `wasm-smoke-${mode}-attempt-${attempt}.png`);
  await Promise.allSettled([
    page.screenshot({ path: attemptPath, fullPage: true }),
    page.screenshot({ path: canonicalScreenshotPath, fullPage: true }),
  ]);
}

async function verifyLaunch(browser, baseUrl, attempt) {
  const mode = liveBaseUrl ? "live" : "local";
  const context = await browser.newContext({
    deviceScaleFactor: 1,
    extraHTTPHeaders: liveBaseUrl
      ? { "cache-control": "no-cache", pragma: "no-cache" }
      : {},
    viewport: { width: 1280, height: 720 },
  });
  const page = await context.newPage();
  let primaryError;

  try {
    const failures = [];
    const responseChecks = [];
    const observedScripts = new Set();
    const observedWasm = new Set();
    const expectedPrefix = baseUrl.pathname;

    page.on("pageerror", error => failures.push(`Page error: ${error.message}`));
    page.on("console", message => {
      const messageText = message.text();
      if (
        message.type() === "error" ||
        (message.type() === "warning" && messageText.startsWith("ERROR:"))
      ) {
        failures.push(`Console ${message.type()}: ${messageText}`);
      }
    });
    page.on("requestfailed", request => {
      const requestUrl = new URL(request.url());
      if (requestUrl.origin === baseUrl.origin) {
        failures.push(
          `Request failed: ${requestUrl.pathname} ` +
          `(${request.failure()?.errorText ?? "unknown error"})`,
        );
      }
    });
    page.on("response", response => {
      responseChecks.push((async () => {
        const responseUrl = new URL(response.url());
        if (responseUrl.origin !== baseUrl.origin) {
          return;
        }
        if (response.status() >= 400) {
          failures.push(`HTTP ${response.status()}: ${responseUrl.pathname}`);
        }
        if (!responseUrl.pathname.startsWith(expectedPrefix)) {
          failures.push(`Asset escaped ${expectedPrefix}: ${responseUrl.pathname}`);
        }
        if (responseUrl.pathname.endsWith(".js")) {
          observedScripts.add(responseUrl.pathname);
        }
        if (responseUrl.pathname.endsWith(".wasm")) {
          observedWasm.add(responseUrl.pathname);
          const headers = await response.allHeaders();
          if (!headers["content-type"]?.startsWith("application/wasm")) {
            failures.push(
              `Incorrect WASM content type: ${headers["content-type"] ?? "missing"}`,
            );
          }
        }
      })());
    });

    const navigationUrl = revisionUrl(baseUrl, "", attempt);
    const navigation = await page.goto(navigationUrl.href, {
      waitUntil: "domcontentloaded",
      timeout: 30_000,
    });
    assert(navigation, "Navigation did not return a response");
    assert.equal(navigation.status(), 200, `Navigation returned HTTP ${navigation.status()}`);
    assert.equal(await page.title(), "AlgoBuddy \u2014 NeetCode Roadmap Visualizer");

    await page
      .locator('#the_canvas_id[data-algobuddy-ready="true"]')
      .waitFor({ state: "attached", timeout: 30_000 });
    await page.locator("#loading_text").waitFor({ state: "detached", timeout: 30_000 });
    await page.waitForLoadState("networkidle", { timeout: 5_000 }).catch(() => {});
    await page.evaluate(() => new Promise(resolve => {
      requestAnimationFrame(() => requestAnimationFrame(resolve));
    }));
    await page.waitForTimeout(500);

    const canvas = page.locator("#the_canvas_id");
    assert.equal(await canvas.count(), 1, "Expected one AlgoBuddy canvas");
    const canvasState = await canvas.evaluate(element => {
      const bounds = element.getBoundingClientRect();
      const style = getComputedStyle(element);
      return {
        devicePixelRatio,
        display: style.display,
        height: element.height,
        opacity: Number.parseFloat(style.opacity),
        renderedHeight: bounds.height,
        renderedWidth: bounds.width,
        visibility: style.visibility,
        width: element.width,
      };
    });

    assert(canvasState.width > 300 && canvasState.height > 150, "Canvas kept default dimensions");
    assert(canvasState.renderedWidth > 0 && canvasState.renderedHeight > 0, "Canvas is not visible");
    assert.notEqual(canvasState.display, "none", "Canvas display is disabled");
    assert.notEqual(canvasState.visibility, "hidden", "Canvas visibility is hidden");
    assert(canvasState.opacity > 0, "Canvas opacity is zero");

    const widthDifference = Math.abs(
      canvasState.width - canvasState.renderedWidth * canvasState.devicePixelRatio,
    );
    const heightDifference = Math.abs(
      canvasState.height - canvasState.renderedHeight * canvasState.devicePixelRatio,
    );
    assert(widthDifference <= 2, "Canvas backing width does not match its rendered width");
    assert(heightDifference <= 2, "Canvas backing height does not match its rendered height");

    const completedResponseChecks = await Promise.allSettled(responseChecks);
    for (const result of completedResponseChecks) {
      if (result.status === "rejected") {
        failures.push(`Response check failed: ${result.reason}`);
      }
    }
    assert(observedScripts.size > 0, "No JavaScript bundle was loaded");
    assert(observedWasm.size > 0, "No WebAssembly bundle was loaded");

    if (expectedRevision) {
      const pageRevision = await page
        .locator('meta[name="algobuddy-deploy-sha"]')
        .getAttribute("content");
      assert.equal(pageRevision, expectedRevision, "Loaded HTML revision did not match");

      const markerResponse = await context.request.get(
        revisionUrl(baseUrl, "deploy-sha.txt", attempt).href,
      );
      assert.equal(markerResponse.status(), 200, "Deployment revision marker was not served");
      assert.equal(
        (await markerResponse.text()).trim(),
        expectedRevision,
        "Deployment revision marker did not match",
      );
    }

    assert.deepEqual(failures, [], failures.join("\n"));
  } catch (error) {
    primaryError = error;
    await saveFailureScreenshots(page, mode, attempt);
  }

  const [contextCleanup] = await Promise.allSettled([context.close()]);
  if (!primaryError && contextCleanup.status === "rejected") {
    primaryError = contextCleanup.reason;
  }
  if (primaryError) {
    throw primaryError;
  }
}

async function runSmokeTest() {
  await resolveExpectedRevision();
  const localServer = liveBaseUrl ? null : await createStaticServer();
  const baseUrl = liveBaseUrl ? normalizeBaseUrl(liveBaseUrl) : localServer.baseUrl;
  let browser;
  let primaryError;

  try {
    if (liveBaseUrl) {
      await waitForPublishedRevision(baseUrl);
    }

    browser = await chromium.launch({
      headless: true,
      args: ["--enable-unsafe-swiftshader"],
    });

    const attempts = liveBaseUrl ? liveLaunchAttempts : 1;
    let launchSucceeded = false;
    for (let attempt = 1; attempt <= attempts; attempt += 1) {
      try {
        await verifyLaunch(browser, baseUrl, attempt);
        console.log(`AlgoBuddy launched successfully from ${baseUrl.href}`);
        launchSucceeded = true;
        break;
      } catch (error) {
        if (attempt === attempts) {
          throw error;
        }
        console.log(
          `Published app launch attempt ${attempt} of ${attempts} failed: ` +
          `${error instanceof Error ? error.message : error}`,
        );
        await delay(retryDelayMs);
      }
    }
    assert(launchSucceeded, "WebAssembly launch attempts completed without success");
  } catch (error) {
    primaryError = error;
  } finally {
    const cleanupResults = await Promise.allSettled([
      browser ? browser.close() : Promise.resolve(),
      localServer ? localServer.close() : Promise.resolve(),
    ]);
    if (!primaryError) {
      const cleanupFailure = cleanupResults.find(result => result.status === "rejected");
      if (cleanupFailure?.status === "rejected") {
        primaryError = cleanupFailure.reason;
      }
    }
  }

  if (primaryError) {
    throw primaryError;
  }
}

await runSmokeTest();
