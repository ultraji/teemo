(function () {
    window.requests = [];

    // rewrite xhr
    const originalOpen = XMLHttpRequest.prototype.open;
    const originalSend = XMLHttpRequest.prototype.send;

    XMLHttpRequest.prototype.open = function (method, url) {
        this.send = function (body) {
            this.addEventListener("readystatechange", function () {
                if (this.readyState === XMLHttpRequest.DONE) {
                    window.requests.push(
                        {
                            method: method,
                            url: this.responseURL,
                            request: body,
                            response: this.responseText,
                            status: this.status,
                            status_text: this.statusText
                        }
                    );
                }
            });
            originalSend.apply(this, arguments);
        };
        originalOpen.apply(this, arguments);
    };

    // rewrite fetch
    const originFetch = window.fetch

    window.fetch = async (input, init) => {
        return originFetch(input, init).then(async (response) => {
            const resp = await response.clone();
            window.requests.push({
                method: input.method || init.method || 'GET',
                url: input.url || input,
                //request: input.body,
                response: await resp.text(),
                status: resp.status,
                status_text: resp.statusText
            });

            return response;
        });
    };
})();