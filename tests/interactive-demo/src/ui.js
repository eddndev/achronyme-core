
// Elements
const sdkStatusText = document.getElementById('sdk-status-text');
const memoryStatusText = document.getElementById('memory-status-text');
const resultsOutput = document.getElementById('results-output');
const categoryButtons = document.querySelectorAll('.test-category');
const testSections = document.querySelectorAll('.test-section');

export const elements = {
  sdkStatusText,
  memoryStatusText,
  resultsOutput,
  categoryButtons,
  testSections,
  runAllBtn: document.getElementById('run-all'),
  clearResultsBtn: document.getElementById('clear-results'),
  gcNowBtn: document.getElementById('gc-now'),
  resetEvalBtn: document.getElementById('reset-eval'),
  testButtons: document.querySelectorAll('.test-btn'),
};

export function setSDKStatus(status, color) {
  sdkStatusText.textContent = status;
  sdkStatusText.style.color = `var(--${color})`;
}

export function updateMemoryStatus(ach) {
  if (ach) {
    const count = ach.getActiveValuesCount();
    memoryStatusText.textContent = `${count} values`;
  }
}

export function addResult(title, content, type = 'info') {
  const placeholder = resultsOutput.querySelector('.placeholder');
  if (placeholder) {
    placeholder.remove();
  }

  const resultItem = document.createElement('div');
  resultItem.className = `result-item ${type}`;

  const resultHeader = document.createElement('div');
  resultHeader.className = 'result-header';

  const resultTitle = document.createElement('div');
  resultTitle.className = 'result-title';
  resultTitle.textContent = title;

  const resultTime = document.createElement('div');
  resultTime.className = 'result-time';
  resultTime.textContent = new Date().toLocaleTimeString();

  resultHeader.appendChild(resultTitle);
  resultHeader.appendChild(resultTime);

  const resultContent = document.createElement('div');
  resultContent.className = 'result-content';
  resultContent.textContent = content;

  resultItem.appendChild(resultHeader);
  resultItem.appendChild(resultContent);

  resultsOutput.insertBefore(resultItem, resultsOutput.firstChild);
}

export function clearResults() {
  resultsOutput.innerHTML = '<p class="placeholder">Run tests to see results here...</p>';
}

export function setupCategoryNavigation() {
  categoryButtons.forEach(button => {
    button.addEventListener('click', () => {
      const category = button.dataset.category;
      categoryButtons.forEach(btn => btn.classList.remove('active'));
      button.classList.add('active');
      testSections.forEach(section => {
        section.classList.toggle('hidden', section.id !== `section-${category}`);
      });
    });
  });
}
