// Setup three.js WebGL renderer
var renderer = new THREE.WebGLRenderer( { antialias: true } );

// Append the canvas element created by the renderer to document body element.
document.body.appendChild( renderer.domElement );

//Create a three.js scene
var scene = new THREE.Scene();

//Create a three.js camera
var camera = new THREE.PerspectiveCamera( 110, window.innerWidth / window.innerHeight, 2, 10000 );
scene.add(camera);

//Apply VR headset positional data to camera.
var controls = new THREE.VRControls( camera );

//Apply VR stereo rendering to renderer
var effect = new THREE.VREffect( renderer );
effect.setSize( window.innerWidth, window.innerHeight );

/*
Create, position, and add 3d objects
*/
var pi = 3.141592653589793238;


var bars = [];
var heights = [] ;
for (var i = 0; i < 20; i++) {
var height = Math.floor(Math.random() * (100 - 30 + 1) + 30);

  bars[i] = new THREE.Mesh(new THREE.BoxGeometry(10, height, 10), new THREE.MeshBasicMaterial({color: 0x0443EE*i*i + 1000}));
  bars[i].position.z = -70;
  bars[i].position.x = (i - (10) )*(15) ;
  bars[i].position.y = -50 ;
  bars[i].userData = height ;
  heights[i] = bars[i].userData ;
  scene.add(bars[i]);
//  alert(heights[i]) ;
}

var floor = new THREE.Mesh( new THREE.PlaneBufferGeometry( 1000, 1000, 1, 1 ), new THREE.MeshBasicMaterial( { color: 0x404040, side: THREE.DoubleSide } ) );
floor.rotation.x = pi/2;
floor.position.y = -50;
scene.add( floor );
//selectionSort(bars);


//var arr = [7, 4, 3, 5, 8, 6, 20, 12] ;


function swap(items, firstIndex, secondIndex){
    var temp = items[firstIndex];
    items[firstIndex] = items[secondIndex];
    items[secondIndex] = temp;


}


/**
 * A selection sort implementation in JavaScript. The array
 * is sorted in-place.
 * @param {Array} items An array of items to sort.
 * @return {Array} The sorted array.
 */
function selectionSort(items){

    var len = items.length,
        min, i, j;

    for (i=0; i < len; i++){

        // set minimum to this position
        min = i;

        // check the rest of the array to see if anything is smaller
        for (j=i+1; j < len; j++){
            if (items[j].userData < items[min].userData){
                min = j;
            }
        }

        // if the minimum isn't in the position, swap it
        if (i != min){
            swap(items, i, min);
        }
    }


}


function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

var len = bars.length ;
var i = 0 ;
var j ;
var min ;
var aFlag = 1 ;
var x1 ; // for x - coordinate
var x2 ;
var swapFlag ;


async function animate() {

if(i < len && aFlag === 1){


      // set minimum to this position
      min = i;

      // check the rest of the array to see if anything is smaller
      for (j=i+1; j < len; j++){
          if (bars[j].userData < bars[min].userData){
              min = j;
          }
      }

      // if the minimum isn't in the position, swap it

      await sleep(500) ;

        x1 = bars[i].position.x ;


        bars[i].position.set(x1 , -50 , -50) ;



        x2 = bars[min].position.x ;

        bars[min].position.set(x2 , -50 , -50) ;


        controls.update();

        // Render the scene through the VREffect.
        effect.render( scene, camera );
        //requestAnimationFrame( animate );
        await sleep(500) ;

        bars[min].position.set((i - 10 )*15, -50 , -70) ;

        bars[i].position.set((min - 10 )*15, -50 , -70) ;

       swap(bars, i, min);





      i++;

}


      //    bars[min].position.set((i - 5 )*15, -50 , -70) ;

        //  alert('Ready for Swap!');

          /*var temp = items[firstIndex];
          items[firstIndex] = items[secondIndex];
          items[secondIndex] = temp;
          */




  //Update VR headset position and apply to camera.
  controls.update();

  // Render the scene through the VREffect.
  effect.render( scene, camera );
  requestAnimationFrame( animate );
}

animate();	// Kick off animation loop

/*
Listen for click event to enter full-screen mode.
We listen for single click because that works best for mobile for now
*/
document.body.addEventListener( 'click', function(){
  effect.setFullScreen( true );
})

/*
Listen for keyboard events
*/
function onkey(event) {
  event.preventDefault();

  if (event.keyCode == 90) { // z
    controls.resetSensor(); //zero rotation
  } else if (event.keyCode == 70 || event.keyCode == 13) { //f or enter
    effect.setFullScreen(true) //fullscreen
  }
};
window.addEventListener("keydown", onkey, true);

/*
Handle window resizes
*/
function onWindowResize() {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  effect.setSize( window.innerWidth, window.innerHeight );
}
window.addEventListener( 'resize', onWindowResize, false );
