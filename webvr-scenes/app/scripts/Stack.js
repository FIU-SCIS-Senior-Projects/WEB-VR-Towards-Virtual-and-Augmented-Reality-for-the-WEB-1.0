var jQuery = require('jquery');
window.$ = window.jQuery = jQuery;

require('bootstrap');
jQuery.noConflict(true);

console.log('stack page script')

var pinkBox = $("#pink");

var popEl = document.querySelector('pop');
var popCount = 0;
var emphasizeToggle = false;

pinkBox.addEventListener('componentchanged', function(evt) {
  if (evt.detail.name === 'position' && popCount < 6 && emphasizeToggle === false) {
    var event = new Event('emphasize');
    popCount++;
    emphasizeToggle = true;
  }
})

popEl.addEventListener('animation__[pop]-complete', function(evt){
  emphasizeToggle = false;
})