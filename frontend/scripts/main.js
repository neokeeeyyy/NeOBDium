const { listen, emit } = window.__TAURI__.event;
const { save } = window.__TAURI__.dialog;
const { writeFile, removeFile } = window.__TAURI__.fs;
const { appWindow } = window.__TAURI__.window;

import {
  clearDtcs,
  exportDtcs,
  connectElm,
  disconnectElm,
  clearObdView,
  appendTerminalOutput,
  addNotification,
  btnHoldToActivate,
  addCustomPIDRow,
} from "./features.js";

import { saveUnitPreference } from "./settings.js";

let connected = false;
window.connectionConfig = {
  serialPort: "0",
  baudRate: "0",
  protocol: 0,
};

let hideVin = false;
let deleteLogsOnExit = false;
let autoCheckCodes = false;
let autoSaveCodes = false;
let logFilePath = "";

const dropdowns = document.querySelectorAll(".dropdown");
const graphDropdowns = document.querySelectorAll(".graph-dropdown");
const connectButton = document.getElementById("btn-connect");
const disconnectButton = document.getElementById("btn-disconnect");
const clearObdButton = document.getElementById("obd-clear");
const pauseObdButton = document.getElementById("obd-pause");
const dtcClearButton = document.getElementById("dtc-clear-button");

window.addEventListener("DOMContentLoaded", () => {
  emit("frontend-loaded");

  emit("get-serial-ports");

  emit("get-connection-status");
});

function handleDropdown(dropdown, toggleName, menuName) {
  const toggle = dropdown.querySelector(toggleName);
  const menu = dropdown.querySelector(menuName);

  toggle.addEventListener("click", (e) => {
    e.stopPropagation();
    if (menu.style.display === "block") {
      menu.style.display = "none";
    } else {
      document.querySelectorAll(menuName).forEach((m) => {
        m.style.display = "none";
      });
      menu.style.display = "block";
    }
  });

  menu.addEventListener("click", (e) => {
    if (e.target.tagName === "LI") {
      toggle.textContent = e.target.textContent;
      toggle.dataset.value = e.target.dataset.value;
      menu.style.display = "none";

      if (toggle.id == "unit-preference" && window.unitPreferences) {
        const unitType = toggle.getAttribute("data-target");
        const unit = toggle.dataset.value;

        saveUnitPreference(unitType, unit);
      }
    }
  });
}

clearObdButton.addEventListener("click", clearObdView);
pauseObdButton.addEventListener("click", () => {
  window.obdViewPaused = !window.obdViewPaused;
  if (obdViewPaused) {
    pauseObdButton.textContent = "REANUDAR";
  } else {
    pauseObdButton.textContent = "PAUSAR";
  }
});

dropdowns.forEach((dropdown) => {
  if (dropdown.id == "baud-rate-dropdown") {
    return;
  }

  handleDropdown(dropdown, ".dropdown-toggle", ".dropdown-menu");
});

graphDropdowns.forEach((dropdown) => {
  handleDropdown(dropdown, ".graph-dropdown-toggle", ".graph-dropdown-menu");
});

document.addEventListener("click", () => {
  document.querySelectorAll(".dropdown-menu").forEach((menu) => {
    menu.style.display = "none";
  });
});

connectButton.addEventListener("click", async () => {
  const baudRate = document.getElementById("baud-rate-selected");
  const serialPort = document.getElementById("serial-port-selected");
  const protocol = document.getElementById("protocol-selected");

  connectElm(
    baudRate.textContent.trim(),
    serialPort.textContent.trim(),
    parseInt(protocol.dataset.value),
  );
});

disconnectButton.addEventListener("click", disconnectElm);

const dtcScanButton = document.getElementById("dtc-scan-button");
dtcScanButton.addEventListener("click", async () => {
  await new Promise((r) => setTimeout(r, 500));

  addNotification("CÓDIGOS DE ERROR", "Escaneo de códigos de error completado.");

  emit("get-dtcs");
});

const fill = document.getElementById("btn-hold-fill");
btnHoldToActivate(dtcClearButton, fill, clearDtcs);

const dtcLogButton = document.getElementById("dtc-log-file");
const dtcList = document.getElementById("dtc-list");
dtcLogButton.addEventListener("click", () => exportDtcs(false));

const logFileButton = document.getElementById("log-file-button");
logFileButton.addEventListener("click", async () => {
  window.logFilePath = await save({
    title: "Guardar como JSON",
    defaultPath: "requests.json",
    filters: [{ name: "JSON", extensions: ["json"] }],
  });

  if (!window.logFilePath) {
    window.logFilePath = "./requests.json";
    return;
  }

  document.getElementById("log-file-path").textContent = window.logFilePath;
});

const imTestRefreshButton = document.getElementById("readiness-test-refresh");
const imTestExportButton = document.getElementById("readiness-test-export");
const imTestList = document.getElementById("readiness-tests-list");

imTestRefreshButton.addEventListener("click", () => {
  emit("get-readiness-tests");
});

imTestExportButton.addEventListener("click", async () => {
  let totalJSON = [];
  imTestList.childNodes.forEach((testRow) => {
    if (testRow.nodeType == 1) {
      const available =
        testRow.querySelector("#test-availability").textContent.trim() ==
        "COMPLETO"
          ? true
          : false;
      const complete =
        testRow.querySelector("#test-completeness").textContent.trim() ==
        "LISTO"
          ? true
          : false;

      const testJSON = {
        name: testRow
          .querySelector("#test-name")
          .textContent.replace("PRUEBA: ", "")
          .trim(),
        available: available,
        complete: complete,
      };

      totalJSON.push(testJSON);
    }
  });

  const path = await save({
    title: "Guardar como JSON",
    defaultPath: "readiness_tests.json",
    filters: [{ name: "JSON", extensions: ["json"] }],
  });

  if (!path) {
    return;
  }

  await writeFile({ path, contents: JSON.stringify(totalJSON, null, 2) });
});

const vinExportButton = document.getElementById("vin-export");
const vinDetails = document.getElementById("vin-container");

vinExportButton.addEventListener("click", async () => {
  let vinObj = {};
  vinDetails.childNodes.forEach((card) => {
    if (card.nodeType == 1) {
      const key = card
        .querySelector("h3")
        .textContent.replaceAll(" ", "_")
        .toLowerCase();
      const value = card.querySelector(".value").textContent.trim();
      vinObj[key] = value;
    }
  });

  const path = await save({
    title: "Guardar como JSON",
    defaultPath: "vin_details.json",
    filters: [{ name: "JSON", extensions: ["json"] }],
  });

  if (!path) {
    return;
  }

  await writeFile({ path, contents: JSON.stringify(vinObj, null, 2) });
});

const input = document.getElementById("terminal-input");

input.addEventListener("input", (e) => {
  if (!input.value.startsWith("> ")) {
    input.value = "> ";
  }
});

input.addEventListener("keydown", (e) => {
  if (input.selectionStart < 2) {
    e.preventDefault();
  }
});

input.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    if (input.value.trim() === "") {
      return;
    }

    const command = input.value.slice(2).trim();
    if (command.length > 0) {
      appendTerminalOutput(command);
      emit("terminal-command", command);
      input.value = "> ";
    }
    e.preventDefault();
  }
});

const addPidsButton = document.getElementById("pid-add");
addPidsButton.addEventListener("click", () => {
  addCustomPIDRow();
});
