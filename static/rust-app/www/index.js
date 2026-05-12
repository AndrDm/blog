import init, * as wasm from '../pkg/rust_app';

init().then(() => {
  function computePrimes() {
    const input = parseInt(document.getElementById('inputNumber').value);
    if (!isNaN(input) && input >= 1) {
      const primes = wasm.sieve_of_eratosthenes(input);
      document.getElementById('output').innerText = primes.join(', ');
    } else {
      document.getElementById('output').innerText = 'Enter a valid integer.';
    }
  }

  document.getElementById('computeButton')
    .addEventListener('click', computePrimes);
});
