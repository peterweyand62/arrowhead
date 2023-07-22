addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const path = new URL(request.url).pathname;

  try {
    const staticAssets = await WRANGLER_STATIC_SITE.get(path, { type: "arrayBuffer" });

    return new Response(staticAssets, {
      headers: {
        "Content-Type": getContentType(path),
      },
    });
  } catch (error) {
    return new Response("Not Found", { status: 404 });
  }
}

function getContentType(path) {
  const ext = path.split(".").pop();
  
  switch(ext) {
    case "html":
      return "text/html";
    case "css":
      return "text/css";
    case "js":
      return "application/javascript";
    // Add more mime types as needed
  
    default:
      return "text/plain";
  }
}

