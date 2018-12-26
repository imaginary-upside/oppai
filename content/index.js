//const response = await fetch("http://127.0.0.1:10010/get_videos");
//console.log(await response.text());
async function main() {
  //await fetch("/api/scan_videos");

  let http = new XMLHttpRequest();
  http.open("GET", "http://127.0.0.1:10010/api/get_videos");
  http.send();
  http.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      const videos = JSON.parse(http.responseText);
      renderVideos(videos);
    }
  }

  document.getElementById("search_code").addEventListener("search", search);
  document.getElementById("search_title").addEventListener("search", search);

  document.getElementById("scan").addEventListener("click", async () => {
    await fetch("/api/scan_videos");

    let response = await fetch("/api/get_videos/");
    let videos = await response.text();
    renderVideos(JSON.parse(videos));
  });
}
main();

async function search() {
  let codeDoc = document.getElementById("search_code");
  let titleDoc = document.getElementById("search_title");

  console.log(JSON.stringify({ code: codeDoc.value, title: titleDoc.value }));
  
  let response = await fetch("/api/search/", {
    method: "POST",
    body: JSON.stringify({ code: codeDoc.value, title: titleDoc.value })
  });
  let videos = await response.text();
  renderVideos(JSON.parse(videos));
}

function renderVideos(videos) {

  let videosDoc = document.getElementById("videos");
 
  // Delete all existing videos
  while (videosDoc.firstChild) {
    videosDoc.removeChild(videosDoc.firstChild);
  }

  console.log(videos.length);

  videos.forEach(video => {
    let videoDoc = document.createElement("span");
    videoDoc.className = "video";
    videoDoc.innerText = video["code"];
    videoDoc.style.backgroundImage = `url('http://127.0.0.1:10010/assets/${video["cover"]}')`;
    videoDoc.addEventListener("click", () => {
      let http = new XMLHttpRequest();
      http.open("POST", "http://127.0.0.1:10010/api/play_video");
      http.send(video["id"]);
    });
    videosDoc.appendChild(videoDoc);
  });
}
