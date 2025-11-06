import { Achronyme } from '../../dist/sdk/Achronyme.js';
import { elements, setSDKStatus, updateMemoryStatus, addResult, clearResults, setupCategoryNavigation } from './src/ui.js';
import { allTests } from './src/tests/index.js';

let ach = null;

async function initSDK() {
  try {
    setSDKStatus('Initializing...', 'warning');
    ach = new Achronyme();
    await ach.init();
    setSDKStatus('Ready', 'success');
    addResult('SDK Initialized', 'SDK is ready for testing', 'success');
    updateMemoryStatus(ach);
  } catch (error) {
    console.error('❌ SDK initialization failed:', error);
    setSDKStatus('Error', 'error');
    addResult('SDK Initialization Failed', `${error.message}\n\nCheck browser console for details`, 'error');
  }
}

function forceGC() {
  if (!ach) return;
  const freed = ach.gc();
  addResult('Manual GC', `Freed ${freed} handles`, 'info');
  updateMemoryStatus(ach);
}

function resetEvaluator() {
  if (!ach) return;
  ach.resetEvaluator();
  addResult('Reset Evaluator', 'SOC evaluator state cleared', 'info');
}

async function runTest(testName, testFn, button) {
  if (!ach) {
    addResult(testName, 'SDK not initialized', 'error');
    return;
  }

  button.classList.add('running');
  const startTime = performance.now();

  try {
    const result = await testFn(ach);
    const duration = (performance.now() - startTime).toFixed(2);
    button.classList.remove('running');
    button.classList.add('success');
    setTimeout(() => button.classList.remove('success'), 2000);
    addResult(testName, `✓ ${result}\nTime: ${duration}ms`, 'success');
  } catch (error) {
    const duration = (performance.now() - startTime).toFixed(2);
    button.classList.remove('running');
    button.classList.add('error');
    setTimeout(() => button.classList.remove('error'), 2000);
    addResult(testName, `✗ ${error.message}\nTime: ${duration}ms`, 'error');
  } finally {
    updateMemoryStatus(ach);
  }
}

function attachEventListeners() {
  elements.clearResultsBtn.addEventListener('click', clearResults);
  elements.gcNowBtn.addEventListener('click', forceGC);
  elements.resetEvalBtn.addEventListener('click', resetEvaluator);

  elements.testButtons.forEach(button => {
    button.addEventListener('click', async () => {
      const testName = button.dataset.test;
      const testFn = allTests[testName];
      if (testFn) {
        await runTest(testName, testFn, button);
      } else {
        addResult(testName, 'Test not implemented yet', 'error');
      }
    });
  });

  elements.runAllBtn.addEventListener('click', async () => {
    const activeSection = document.querySelector('.test-section:not(.hidden)');
    if (!activeSection) return;

    const categoryTests = activeSection.querySelectorAll('.test-btn');
    addResult('Running All Tests', `Starting ${categoryTests.length} tests in category...`, 'info');

    for (const button of categoryTests) {
      const testName = button.dataset.test;
      const testFn = allTests[testName];
      if (testFn) {
        await runTest(testName, testFn, button);
        await new Promise(resolve => setTimeout(resolve, 100));
      }
    }
    addResult('All Tests Complete', 'All tests in category finished', 'success');
  });
}

// --- App Entry Point ---
function main() {
  setupCategoryNavigation();
  attachEventListeners();
  initSDK();
}

if (document.readyState === 'loading') {
  window.addEventListener('DOMContentLoaded', main);
} else {
  main();
}