

var demo = new CANNON.Demo();
var size = 2, mass=5;
function createTetra(){
      var verts = [new CANNON.Vec3(0,0,0),
                   new CANNON.Vec3(2,0,0),
                   new CANNON.Vec3(0,2,0),
                   new CANNON.Vec3(0,0,2)];
      var offset = -0.35;
      for(var i=0; i<verts.length; i++){
          var v = verts[i];
          v.x += offset;
          v.y += offset;
          v.z += offset;
      }
      return new CANNON.ConvexPolyhedron(verts,
                                          [
                                              [0,3,2], // -x
                                              [0,1,3], // -y
                                              [0,2,1], // -z
                                              [1,2,3], // +xyz
                                          ]);
}
function createBoxPolyhedron(size){
    size = size || 1;
    var box = new CANNON.Box(new CANNON.Vec3(size,size,size));
    return box.convexPolyhedronRepresentation;
}
function createCompound(mass){
  var compoundBody = new CANNON.Body({ mass: mass });
  var s = size;
  var shape = new CANNON.Box(new CANNON.Vec3(0.5*s,0.5*s,0.5*s));
  compoundBody.addShape(shape, new CANNON.Vec3( 0, 0, s));
  compoundBody.addShape(shape, new CANNON.Vec3( 0, 0, 0));
  compoundBody.addShape(shape, new CANNON.Vec3( 0, 0,-s));
  compoundBody.addShape(shape, new CANNON.Vec3(-s, 0,-s));
  return compoundBody;
}
var j = 0 ; // global counter for stack
var k = 0 ; // global counter for enqueue
var h = 0 ; // head pointer for queue
var hashNumber = 0 ;
// create 10 x 10 hash table for seperate chaining
var hashTable = new Array();
for (var x = 0; x < 10; x++) {
hashTable[x] = new Array();
}
var hashKey ;
var i ;

// Stack Scene
demo.addScene("Stack Push",function(){

var world = setupWorld(demo);
var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
//var sceneStack = new Stack();
//  var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
j++;
//  pushScene(sceneStack, boxShape, size, mass);
// Below is a test loop to make sure that the boxes get created correctly
for(i = 0 ; i < j ; i++){
    var b1 = new CANNON.Body({ mass: 5 });
    b1.addShape(boxShape);
    b1.position.set(0,i*(-2)*(size), 0);
    world.addBody(b1);
//      sceneStack.push(3) ;
    demo.addVisual(b1);
  //  scene.remove(b1) ;
}
});
// Stack Pop Scene
demo.addScene("Stack Pop",function(){
  if(j === 0){
    alert('Stack is empty!');
  } else{
    j--;
    var world = setupWorld(demo);
    var world = setupWorld(demo);
   var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var i ;
      for(i = 0 ; i < j ; i++){
              var b1 = new CANNON.Body({ mass: 5 });
              b1.addShape(boxShape);
              b1.position.set(0,i*(-2)*(size), 0);
              world.addBody(b1);
              demo.addVisual(b1);
          }
            }
  });
// Enqueue Scene
demo.addScene("Queue Enqueue",function(){
    k++;
    var world = setupWorld(demo);
   var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    //  var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));

  //  pushScene(sceneStack, boxShape, size, mass);
  // Below is a test loop to make sure that the boxes get created correctly
    for(i = h ; i < k ; i++){
            var b1 = new CANNON.Body({ mass: 5 });
            b1.addShape(boxShape);
            b1.position.set(0,i*(-2)*(size), 0);
            world.addBody(b1);
            demo.addVisual(b1);
          //  scene.remove(b1) ;
    }
    for( i = 0 ; i < h ; i++){
           var p = new CANNON.Body({ mass: 1 });
           p.addShape(new CANNON.Particle());
           p.position.set(0,i*(-2)*size,0);
           world.addBody(p);
           demo.addVisual(p);
     }
  });
demo.addScene("Dequeue",function(){
    if(h >= k){
      alert('Nothing to Dequeue! Queue is empty');
      var world = setupWorld(demo);
    }
    else{
     h++;
    var world = setupWorld(demo);
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
     //  var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));

       for(i = h ; i < k ; i++){
              var b1 = new CANNON.Body({ mass: 5 });
              b1.addShape(boxShape);
              b1.position.set(0,i*(-2)*(size), 0);
              world.addBody(b1);
              demo.addVisual(b1);
              }
       for( i = 0 ; i < h ; i++){
              var p = new CANNON.Body({ mass: 1 });
              p.addShape(new CANNON.Particle());
              p.position.set(0,i*(-2)*size,0);
              world.addBody(p);
              demo.addVisual(p);
        }
      }
  });
  /*Collision function: h(K) = K mod 10
    All keys are perfect squares*/
demo.addScene("Hash-Table Insert",function(){
    var cur ;
    var world = setupWorld(demo);
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
     //  var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var sphereShape = new CANNON.Sphere(size*0.5);
     // counter for amount of boxes in the scene
     hashKey = hashNumber*hashNumber ;
     var hashIndex = hashKey % 10 ;
     if(hashTable[hashIndex].length < 10){
     hashTable[hashIndex].push(hashKey) ;
    }
    // initialize table for seperate chaining
    for(iter = 0 ; iter < hashTable.length ; iter++){
           var b1 = new CANNON.Body({ mass: 5 });
           b1.addShape(boxShape);
           b1.position.set(0,iter*(-2)*(size), 0);
           world.addBody(b1);
           demo.addVisual(b1);
          for(cur = 0; cur < hashTable[iter].length ; cur++ ){
             // Sphere
             var b = new CANNON.Body({ mass: 5 });
             b.addShape(sphereShape);
             b.position.set(cur*(-2)*(size) - 2,iter*(-2)*(size),0);
             world.addBody(b);
             demo.addVisual(b);
           } // end of inner loop
      } //end of outer loop
          hashNumber++ ;
  });
demo.addScene("Binary Search Tree",function(){
    var world = setupWorld(demo);
    // Sphere
    var sphereShape = new CANNON.Sphere(size*0.5);
    var b = new CANNON.Body({ mass: 5 });
    b.addShape(sphereShape);
    b.position.set(0,0,size);
    world.addBody(b);
    demo.addVisual(b);
    // Particle
    var p = new CANNON.Body({ mass: 1 });
    p.addShape(new CANNON.Particle());
    p.position.set(0.02,0,3*size);
    world.addBody(p);
    demo.addVisual(p);
  });
demo.addScene("Breadth-First Search",function(){
    var world = setupWorld(demo);
    var boxShape = new CANNON.Box(new CANNON.Vec3(size,size,size));
    var b1 = new CANNON.Body({ mass: 5 });
    b1.addShape(boxShape);
    b1.position.set(0,0,1*size);
    world.addBody(b1);
    demo.addVisual(b1);
  });
demo.addScene("Depth-First Search",function(){
    var world = setupWorld(demo);
    var b1 = createCompound(5);
    b1.position.set(0,0,4*size);
    world.addBody(b1);
    demo.addVisual(b1);
  });
demo.addScene("Sorting",function(){
    var world = setupWorld(demo);
    var shape = createTetra();
    var b1 = new CANNON.Body({ mass: 5 });
    b1.addShape(shape);
    b1.position.set(0,0,1*size);
    world.addBody(b1);
    demo.addVisual(b1);
  });
demo.addScene("plane/particle",function(){
    var world = setupWorld(demo);
    // Particle
    var p = new CANNON.Body({ mass: 1 });
    p.addShape(new CANNON.Particle());
    p.position.set(0.02,0,3*size);
    world.addBody(p);
    demo.addVisual(p);
  });
// Boxes
demo.addScene("box/box",function(){
    var world = setupWorld(demo);
    // Box 1
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var b1 = new CANNON.Body({ mass: 5 });
    b1.addShape(boxShape);
    b1.position.set(0,0,1*size);
    world.addBody(b1);
    demo.addVisual(b1);
    // Box 2
    var b2 = new CANNON.Body({ mass: 5 });
    b2.addShape(boxShape);
    b2.position.set(size*0.5,0,3*size);
    world.addBody(b2);
    demo.addVisual(b2);
  });
demo.addScene("box/compound",function(){
    var world = setupWorld(demo);
    var b2 = createCompound(5);
    b2.position.set(size*0.5,0,2*size);
    world.addBody(b2);
    demo.addVisual(b2);
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var b1 = new CANNON.Body({ mass: 5 });
    b1.addShape(boxShape);
    b1.position.set(0,0,7*size);
    world.addBody(b1);
    demo.addVisual(b1);
  });
demo.addScene("box/convex",function(){
    var world = setupWorld(demo);
    var shape = createTetra(size);
    var b2 = new CANNON.Body({ mass: 5 });
    b2.addShape(shape);
    b2.position.set(0,0,5*size);
    world.addBody(b2);
    demo.addVisual(b2);
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var b1 = new CANNON.Body({ mass: 5 });
    b1.addShape(boxShape);
    b1.position.set(0,0,2*size);
    world.addBody(b1);
    demo.addVisual(b1);
  });
demo.addScene("box/particle",function(){
    var world = setupWorld(demo);
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var b1 = new CANNON.Body({ mass:5 });
    b1.addShape(boxShape);
    b1.position.set(0,0,1*size);
    world.addBody(b1);
    demo.addVisual(b1)
    // Particle
    var p = new CANNON.Body({ mass: 1 });
    p.addShape(new CANNON.Particle());
    p.position.set(0,0,3*size);
    world.addBody(p);
    demo.addVisual(p);
  });
demo.addScene("compound/compound",function(){
    var world = setupWorld(demo);
    var b2 = createCompound(5);
    b2.position.set(size*0.5,0,6*size);
    world.addBody(b2);
    demo.addVisual(b2);
    var b2 = createCompound(5);
    b2.position.set(size*0.5,0,2*size);
    world.addBody(b2);
    demo.addVisual(b2);
  });
demo.addScene("compound/convex",function(){
    var world = setupWorld(demo);
    var tetraShape = createTetra();
    var b1 = new CANNON.Body({ mass:5 });
    b1.addShape(tetraShape);
    b1.position.set(0,0,3*size);
    world.addBody(b1);
    demo.addVisual(b1);
    // Box 2
    var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
    var b2 = new CANNON.Body({ mass:5 });
    b2.addShape(boxShape);
    b2.position.set(0,0,1*size);
    world.addBody(b2);
    demo.addVisual(b2);
  });
demo.addScene("compound/particle",function(){
    var world = setupWorld(demo);
    var t = createCompound(5);
    t.position.set(0,0,4*size);
    // Particle
    var p = new CANNON.Body({ mass: 1 });
    p.addShape(new CANNON.Particle());
    p.position.set(0,0,7*size);
    world.addBody(t);
    demo.addVisual(t);
    world.addBody(p);
    demo.addVisual(p);
  });
// ConvexPolyhedron and box
demo.addScene("convex/convex",function(){
    var world = setupWorld(demo);
    var tetraShape = createTetra();
    var b1 = new CANNON.Body({ mass:5 });
    b1.addShape(tetraShape);
    b1.position.set(0.1,0.1,5*size);
    world.addBody(b1);
    demo.addVisual(b1);
    var tetraShape = createTetra();
    var b1 = new CANNON.Body({ mass:5 });
    b1.addShape(tetraShape);
    b1.position.set(0,0,3*size);
    world.addBody(b1);
    demo.addVisual(b1);
  });
// ConvexPolyhedron and particle
demo.addScene("convex/particle",function(){
    var world = setupWorld(demo);
    var tetraShape = createBoxPolyhedron(size);
    var t = new CANNON.Body({ mass:5 });
    t.addShape(tetraShape);
    t.position.set(0,0,1*size);
    t.quaternion.setFromAxisAngle(new CANNON.Vec3(1,0,0),Math.PI/2);
    // Particle
    var p = new CANNON.Body({ mass: 1 });
    p.addShape(new CANNON.Particle());
    p.position.set(0,0,3*size);
    world.addBody(t);
    demo.addVisual(t);
    world.addBody(p);
    demo.addVisual(p);
  });
demo.start();
function setupWorld(demo){
  var world = demo.getWorld();
  world.gravity.set(0,0,-10);
  world.broadphase = new CANNON.NaiveBroadphase();
  world.solver.iterations = 20;
  world.defaultContactMaterial.contactEquationStiffness = 1e7;
  world.defaultContactMaterial.contactEquationRelaxation = 5;
  // ground plane
  var groundShape = new CANNON.Plane();
  var groundBody = new CANNON.Body({ mass:0 });
  groundBody.addShape(groundShape);
  groundBody.position.set(0,0,-1);
  //groundBody.quaternion.setFromAxisAngle(new CANNON.Vec3(0,1,0),0.2);
  world.addBody(groundBody);
  demo.addVisual(groundBody);
  return world;
}
