//const response = await fetch("http://127.0.0.1:10010/get_videos");
//console.log(await response.text());
let http = new XMLHttpRequest();
http.open("GET", "http://127.0.0.1:10010/api/get_videos");
http.send();
http.onreadystatechange = function() {
  if (this.readyState == 4 && this.status == 200) {
    const videos = JSON.parse(http.responseText);
    renderVideos(videos);
  }
}

function renderVideos(videos) {
  let content = document.getElementById("content");
  videos.forEach(video => {
    let videoDoc = document.createElement("span");
    videoDoc.className = "video";
    videoDoc.innerText = video["code"];
    videoDoc.addEventListener("click", () => {
      let http = new XMLHttpRequest();
      http.open("POST", "http://127.0.0.1:10010/api/play_video");
      http.send(video["code"]);
    });
    content.appendChild(videoDoc);
  });
}
