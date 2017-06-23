console.log("BinarySearchTree script")


AFRAME.registerComponent('do-something-once-loaded', {
  init: function() {
    console.log('I am ready!');
  }
});

var sceneEl = document.querySelector('a-scene');

var entityEl = document.createElement('a-entity');
sceneEl.appendChild(entityEl);