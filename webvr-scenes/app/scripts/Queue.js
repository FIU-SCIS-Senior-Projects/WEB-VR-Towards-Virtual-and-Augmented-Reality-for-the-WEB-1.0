console.log('queue page script')


// QUEUE Object Definition

var Queue = function() {
  this.first = null;
  this.last = null;
  this.size = 0;
};

var Node = function(data) {
  this.data = data;
  this.next = null;
};

Queue.prototype.enqueue = function(data) {
  var node = new Node(data);

  if (!this.first){ // for empty list first and last are the same
    this.first = node;
    this.last = node;
  } else { // otherwise we stick it on the end
    this.last.next=node;
    this.last=node;
  }

  this.size += 1;
  return node;
};

Queue.prototype.dequeue = function() {
  if (!this.first) //check for empty list
    return null;

  temp = this.first; // grab top of list
  if (this.first==this.last) {
    this.last=null;  // when we need to pop the last one
  }
  this.first = this.first.next; // move top of list down
  this.size -= 1;
  return temp;
};


function create(squeue){
scenequeue = new Queue() ;
//alert("Queue Scene created!");
}


function eqscene(scenequeue){
var data = document.getElementById("input").value ;
//alert(data);
scenequeue.enqueue(data) ;


alert(scenequeue.toSource());
}

function dqscene(scenequeue){

scenequeue.dequeue();
alert(scenequeue.toSource());
}
