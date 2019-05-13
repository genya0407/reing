(function() {
    Array.from(document.getElementsByTagName('form'))
        .flatMap(form => Array.from(form.elements))
        .filter(elem => elem.type === 'submit')
        .forEach((button) => button.addEventListener('click', () => button.disabled = true));
})()