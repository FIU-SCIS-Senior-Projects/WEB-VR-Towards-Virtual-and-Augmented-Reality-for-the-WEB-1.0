var jQuery = require('jquery');
window.$ = window.jQuery = jQuery;

global.Tether = require('tether');
require('bootstrap');

$('body').scrollspy({selector:'.scrollclass',offset:65});


// jQuery.noConflict(true);

//require('jquery.easing')(jQuery);

console.log('index page script');


// //jQuery to collapse the navbar on scroll
// $(window).scroll(function() {
//     if ($(".navbar").offset().top > 50) {
//         $(".navbar-fixed-top").addClass("top-nav-collapse");
//     } else {
//         $(".navbar-fixed-top").removeClass("top-nav-collapse");
//     }
// });

// //jQuery for page scrolling feature - requires jQuery Easing plugin
// $(function() {
//     $(document).on('click', 'a.page-scroll', function(event) {
//         var $anchor = $(this);
//         $('html, body').stop().animate({
//             scrollTop: $($anchor.attr('href')).offset().top
//         }, 1500, 'easeInOutExpo');
//         event.preventDefault();
//     });
// });

