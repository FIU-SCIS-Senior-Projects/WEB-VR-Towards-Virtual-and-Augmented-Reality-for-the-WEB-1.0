var AFRAME = require('aframe');
var physics = require('aframe-physics-system');
physics.registerAll();

var gradientSky = require('aframe-gradient-sky').component;
var draw = require('aframe-draw-component').component;
var textwrap = require('aframe-textwrap-component').component;
var animation = require('aframe-animation-component');
var look_at = require('aframe-look-at-component');

AFRAME.registerComponent("draw", draw);
AFRAME.registerComponent("textwrap", textwrap);


//

/**/


AFRAME.registerComponent('random-color', {
  dependencies: ['material'],

  init: function () {
    // Set material component's color property to a random color.
    this.el.setAttribute('material', 'color', getRandomColor());
  }
});

function getRandomColor() {
  const letters = '0123456789ABCDEF';
  var color = '#';
  for (var i = 0; i < 6; i++ ) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

/**
 * Listen to an event.
 * When that event is emitted, emit an event on another entity.
 */
AFRAME.registerComponent('event-proxy', {
  schema: {
    listen: {default: ''},
    target: {type: 'selector'},
    emit: {default: ''}
  },

  update: function () {
    var data = this.data;
    this.el.addEventListener(data.listen, function () {
      data.target.emit(data.emit);
    });
  }
});



//
