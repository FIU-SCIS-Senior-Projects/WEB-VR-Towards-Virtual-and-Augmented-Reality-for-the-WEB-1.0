var AFRAME = require('aframe');

var gradientSky = require('aframe-gradient-sky');
AFRAME.registerComponent("gradientsky", gradientSky);

var draw = require('aframe-draw-component').component;
var textwrap = require('aframe-textwrap-component').component;
AFRAME.registerComponent("draw", draw);
AFRAME.registerComponent("textwrap", textwrap);



