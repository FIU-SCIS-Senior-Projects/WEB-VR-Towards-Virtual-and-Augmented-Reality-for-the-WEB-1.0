console.log('Stack Script');


var Stack = function(){
  this.top = null;
  this.size = 0;
};

var Node = function(data){
  this.data = data;
  this.previous = null;
};

Stack.prototype.push = function(data) {
  var node = new Node(data);

  node.previous = this.top;
  this.top = node;
  this.size += 1;
  return this.top;
};

Stack.prototype.pop = function() {
  temp = this.top;
  this.top = this.top.previous;
  this.size -= 1;
  return temp;
};

//Creates stack for the scene

function createStack(){

sstack = new Stack();

}

function pushScene(stackName, data){

alert('Call successful');
// var data = document.getElementById("input").value ;
/*
var boxShape = new CANNON.Box(new CANNON.Vec3(size*0.5,size*0.5,size*0.5));
//sceneStack.push() ;
var b1 = new CANNON.Body({ mass: 5 });
b1.addShape(boxShape);
b1.position.set(0,i*2*(size), 0);
world.addBody(b1);
sceneStack.push(b1) ;
demo.addVisual(b1);
*/

};

function popScene(){

sstack.pop() ;
};

function refTest(){
alert('Script ref correct');

};
