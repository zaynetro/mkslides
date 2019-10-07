;(function () {
    document.addEventListener('keyup', function (e) {
        var currentSlide = parseInt(location.hash.slice(7), 10) || 0;

        if (e.keyCode === 40 || e.keyCode === 39) {
            // Key down or key right
            e.preventDefault();

            var nextId = 'slide-' + (currentSlide + 1);
            if (document.getElementById(nextId)) {
                location.hash = nextId;
            }
        }

        if (e.keyCode === 38 || e.keyCode === 37) {
            // Key up or key left
            e.preventDefault();

            if (currentSlide > 1) {
                var previousId = 'slide-' + (currentSlide - 1);
                location.hash = previousId;
            } else {
                location.hash = 'intro';
            }
        }
    });

})();
