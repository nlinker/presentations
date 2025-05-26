/*****************************************************************
** Author: I'd raather not say ;-)
**
** A plugin adding Typed.js typerwriter text effect to Reveal.js presentations.
**
** Version: 0.1
**
** Portions borrowed from Martijn De Jongh (Martino), martijn.de.jongh@gmail.com
** https://github.com/Martinomagnifico
******************************************************************/

window.RevealTypewriter = window.RevealTypewriter || {
    id: 'RevealTypewriter',
    init: function (deck) {
        initRevealTypewriter(deck);
    }
};

let initRevealTypewriter = function (Reveal) {
    const defaultOptions = {
        stringsElement: '#typewriter-text',
        typeSpeed: 70,
        showCursor: true,
        cursorChar: '_',
        autoInsertCss: true
    };
    let options = Reveal.getConfig().typewriter || {};
    let defaults = function defaults(options, defaultOptions) {
        for (var i in defaultOptions) {
            if (!options.hasOwnProperty(i)) {
                options[i] = defaultOptions[i];
            }
        }
    };
    defaults(options, defaultOptions);
    console.log(options);
    var typed = new Typed('#typed', options);
};
