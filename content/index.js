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

  document.getElementById("search_video").addEventListener("search", search);
  document.getElementById("search_actress").addEventListener("search", search);
  document.getElementById("search_tags").addEventListener("search", search);

  document.getElementById("scan").addEventListener("click", async () => {
    await fetch("/api/scan_videos");

    let response = await fetch("/api/get_videos/");
    let videos = await response.text();
    renderVideos(JSON.parse(videos));
  });
}
main();

async function search() {
  let videoDoc = document.getElementById("search_video");
  let actressDoc = document.getElementById("search_actress");
  let tagsDoc = document.getElementById("search_tags");
  let body = { video: videoDoc.value, actress: actressDoc.value, tags: tagsDoc.value }
  let uri = "/api/search/";
  if (body.video == "" && body.actress == "" && body.tags == "") {
    uri = "/api/get_videos";
  }
  
  let response = await fetch(uri, {
    method: "POST",
    body: JSON.stringify(body)
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

  videos.forEach(video => {
    let videoDoc = document.createElement("span");
    videoDoc.className = "video";

    let imgDoc = document.createElement("img");
    imgDoc.setAttribute("src", "http://127.0.0.1:10010/assets/" + video["cover"]);
    videoDoc.appendChild(imgDoc);

    let titleDoc = document.createElement("div");
    titleDoc.innerText = video["code"];
    titleDoc.setAttribute("class", "title");
    videoDoc.appendChild(titleDoc);

    videoDoc.addEventListener("click", () => {
      let http = new XMLHttpRequest();
      http.open("POST", "http://127.0.0.1:10010/api/play_video");
      http.send(video["id"]);
    });
    videosDoc.appendChild(videoDoc);

    videoDoc.addEventListener("contextmenu", async e => {
      e.preventDefault();

      if (videoDoc.querySelector("ul") !== null) {
        videoDoc.querySelector("ul").classList.toggle("hidden");
      } else {
        let response = await fetch("/api/video_details/", {
          method: "POST",
          body: videoDoc.querySelector(".title").innerText
        });
        let details = JSON.parse(await response.text());
        
        let list = document.createElement("ul");
        videoDoc.appendChild(list);
        details[1].forEach(actress => {
          let li = document.createElement("li");
          li.innerText = actress["name"]

          if (actress["birthdate"] !== "NULL") {
            let birthdate = moment(actress["birthdate"]);
            let releaseDate = moment(details[0]["release_date"]);
            li.innerText += " (" + releaseDate.diff(birthdate, "years") + ")";
          }

          list.appendChild(li);
        });
      }
    });
  });
}
