for (const script of [
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js",
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/javascript.min.js",
    "https://cdnjs.cloudflare.com/ajax/libs/js-beautify/1.14.11/beautify.min.js",
]) {
    const elem = document.createElement('script');
    elem.src = script;
    elem.defer = true;
    document.head.appendChild(elem);
}

const highlightStyle = document.createElement('link');
highlightStyle.rel = "stylesheet";
highlightStyle.href = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css"
document.head.appendChild(highlightStyle);

window.addEventListener('load', () => {
    const input = document.querySelector('#input');

    function queryOrMake(id, type) {
        let result = document.querySelector(`#${id}`);
        if (!result) {
            result = document.createElement(type);
            result.id = id;
            input.before(result);
        }
        return result;
    }

    const resultElem = queryOrMake('result', 'p');

    window.result = function result(r) {
        resultElem.innerText = r;
    }

    const outputArea = queryOrMake('output', 'div');
    const errorArea = queryOrMake('error', 'div');

    function printTo(out, area) {
        const outElem = document.createElement('pre');
        outElem.innerText = out;
        area.appendChild(outElem);
    }

    window.print = function print(out) {
        printTo(out, outputArea);
    }
    window.error = function error(out) {
        printTo(out, errorArea);
    }

    /** @type HTMLDivElement */
    const solution = document.querySelector('#solution');

    solution.autocapitalize = 'off';
    solution.spellcheck = false;
    solution.contentEditable = 'plaintext-only';
    solution.classList.add('javascript');

    function oninput() {
        outputArea.innerHTML = '';
        errorArea.innerHTML = '';
        result('');
        try {
            const solve = new Function('input', solution.textContent);
            solve(input.value);
        } catch (e) {
            error(e);
        }
    }
    input.addEventListener('input', oninput);
    solution.addEventListener('input', oninput);
    oninput();

    function onblur() {
        solution.textContent = js_beautify(solution.textContent);
        delete solution.dataset.highlighted;
        hljs.highlightElement(solution);
    }
    solution.addEventListener('blur', onblur);
    solution.textContent = solution.textContent.trim();
    onblur();
});
