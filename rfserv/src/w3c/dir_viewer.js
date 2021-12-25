(() => {
  const currentDirPathName = dirname;
  const fileInfos = files;
  if (!main) var main = document.getElementById("main");
  (() => {
    const h1 = document.createElement("h1");
    h1.innerText = `Index of ${currentDirPathName}`;
    main.appendChild(h1);
  })();

  const table = document.createElement("table");

  (() => {
    const tr = document.createElement("tr");
    ["Name", "Last Modified", "Size"].forEach((tableTitle) => {
      const th = document.createElement("th");
      th.innerText = tableTitle;
      tr.appendChild(th);
    });
    table.appendChild(tr);
  })();

  if (currentDirPathName !== "/") {
    (() => {
      const tr = document.createElement("tr");
      const parentDirPathName = currentDirPathName
        .replace(/[^\/]*$/, "")
        .replace(/(?<=.+)\/$/, "");

      tr.innerHTML += `<th><a href=\"${parentDirPathName}\">Parent Directory</a></th>`;
      tr.innerHTML += "<th></th>";
      tr.innerHTML += "<th>-</th>";
      table.appendChild(tr);
    })();
  }

  if (fileInfos.length !== 0) {
    const formatDate = (date) => {
      const year = date.getFullYear();
      const fmtNum = (num) => {
        let numStr = num.toString();
        if (numStr.length === 1) {
          numStr = `0${numStr}`;
        }
        return numStr;
      };

      let month = date.getMonth() + 1;
      month = fmtNum(month);

      let day = date.getDate();
      day = fmtNum(day);

      let hour = date.getHours();
      hour = fmtNum(hour);

      let minute = date.getMinutes();
      minute = fmtNum(minute);

      return `${year}-${month}-${day} ${hour}:${minute}`;
    };

    const formatBytes = (bytes, decimals = 2) => {
      if (!+bytes) return "0 B";
      const k = 1024;
      const dm = decimals < 0 ? 0 : decimals;
      const sizes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
    };

    for (const fileInfo of fileInfos) {
      const tr = document.createElement("tr");
      let [name, lastModified, size, isDir] = fileInfo;

      const thName = document.createElement("th");
      thName.innerHTML = `<a href=\"${currentDirPathName}/${name}\">${name}</a>`;

      const thLastModified = document.createElement("th");
      thLastModified.innerText = formatDate(new Date(lastModified));

      const thSize = document.createElement("th");
      thSize.innerText = isDir ? "-" : formatBytes(size);

      [thName, thLastModified, thSize].forEach((th) => tr.appendChild(th));

      table.appendChild(tr);
    }
  }

  main.appendChild(table);
})();
