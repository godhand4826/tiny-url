const input = document.querySelector('#input');
const submit = document.querySelector('#submit');
const output = document.querySelector('#output');
const copy = document.querySelector('#copy');
const autoSubmit = document.querySelector('#auto_submit');
const autoCopy = document.querySelector('#auto_copy');

new ClipboardJS('#copy').on('success', (e) => e.clearSelection());

input.addEventListener('keyup', (e) => e.key === 'Enter' && submit.click());
window.addEventListener('paste', (e) => {
    e.preventDefault();
    input.value = (e.clipboardData || window.clipboardData).getData('text');

    autoSubmit.checked && submit.click();
});
submit.addEventListener('click', () => {
    const url = input.value;
    if (!isValidURL(url)) {
        alert('Please enter a valid URL');
        return;
    }

    fetch('/', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ url }),
    })
        .then((response) => response.text())
        .then((data) => {
            output.text = `${window.origin}/${data}`;
            output.href = `${window.origin}/${data}`;
            autoCopy.checked && copy.click();
        })
        .catch((error) => {
            console.error('Error:', error);
            alert('An error occurred, please try again later');
        });
});

function isValidURL(url) {
    try {
        new URL(url);
    } catch (_) {
        return false;
    }
    return true;
}