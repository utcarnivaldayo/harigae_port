
let aModal = document.getElementById("result-a");
let aInput = document.getElementById("input-a");
let bModal = document.getElementById("result-b");
let bInput = document.getElementById("input-b");
let cModal = document.getElementById("result-c");
let cInput = document.getElementById("input-c");
let dModal = document.getElementById("result-d");
let dInput = document.getElementById("input-d");
let eModal = document.getElementById("result-e");
let eInput = document.getElementById("input-e");
let fModal = document.getElementById("result-f");
let fInput = document.getElementById("input-f");

aModal.addEventListener("shown.bs.modal", function () {
    aInput.focus();
});

bModal.addEventListener("shown.bs.modal", function () {
    bInput.focus();
});

cModal.addEventListener("shown.bs.modal", function () {
    cInput.focus();
});

dModal.addEventListener("shown.bs.modal", function () {
    dInput.focus();
});

eModal.addEventListener("shown.bs.modal", function () {
    eInput.focus();
});

fModal.addEventListener("shown.bs.modal", function () {
    fInput.focus();
});