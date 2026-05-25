const connectBtn = document.querySelector("#connectBtn");
const form = document.querySelector("#verifyForm");
const learnerInput = document.querySelector("#learner");
const courseInput = document.querySelector("#courseId");
const evidenceInput = document.querySelector("#evidence");
const preview = document.querySelector("#invokePreview");

connectBtn.addEventListener("click", () => {
  connectBtn.textContent = "Freighter Ready";
});

form.addEventListener("submit", (event) => {
  event.preventDefault();

  const learner = learnerInput.value.trim() || "G...";
  const courseId = courseInput.value || "1";
  const evidence = evidenceInput.value.trim() || "https://github.com/...";

  preview.textContent = `verify(
  learner = "${learner}",
  course_id = ${courseId},
  evidence = "${evidence}"
)`;
});
