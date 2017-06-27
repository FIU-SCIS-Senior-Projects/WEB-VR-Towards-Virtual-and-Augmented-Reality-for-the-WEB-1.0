using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Robot : MonoBehaviour {

    public float movementSpeed = 0.0f;
    public float stateChangeDelay = 0.0f;

    private StateMachine sm;
    private List<State> states;
    private List<Transition> transitions;
    private State currentState;
    private State initialState;

    private enum Status { START, READY, WAIT, RUNNING, TERMINATED, IDLE };
    private Status currentStatus;

    public bool hit = false;
    private bool hatch = false;

    private float startTime;

	// Use this for initialization
	void Start () {
        this.currentStatus = Status.START;

        this.states = new List<State>();
        this.transitions = new List<Transition>();

        // below is for testing purposes
        State I = new State("0", "initial");
        State A = new State("1", "moveForward");
        State B = new State("2", "moveBackward");
        this.states.Add(I);
        this.states.Add(A);
        this.states.Add(B);

        Transition T1 = new Transition("1", I, A);
        Transition T2 = new Transition("2", A, B);
        Transition T3 = new Transition("3", B, A);
        this.transitions.Add(T1);
        this.transitions.Add(T2);
        this.transitions.Add(T3);

        this.initialState = I;
        this.sm = new StateMachine(this.initialState, this.states, this.transitions);
        this.currentStatus = Status.READY;
    }
	
	// Update is called once per frame
	void Update () {
        print("state: " + this.currentState + " - status: " + this.currentStatus);
		switch(currentStatus)
        {
            case Status.READY:        handleReady();        break;
            case Status.RUNNING:      handleRunning();      break;
            case Status.TERMINATED:   handleTerminated();   break;
            case Status.START:        handleStart();        break;
            case Status.WAIT:         handleWait();         break;
            case Status.IDLE:         handleIdle();         break;
        }
	}

    void handleReady()
    {
        this.currentState = this.sm.Next();
        if (this.currentState != null) this.currentStatus = Status.RUNNING;
        else this.currentStatus = Status.IDLE;
    }

    void handleRunning()
    {
        switch(currentState.Type)
        {
            case "moveForward":         moduleMoveForward();    break;
            case "moveBackward":        moduleMoveBackward();   break;
            default: break;
        }
        
    }

    void handleTerminated()
    {
        this.cleanup();
        
        if(Time.time - this.startTime > this.stateChangeDelay)
            this.currentStatus = Status.READY;
    }

    void handleStart() { }
    void handleWait() { }
    void handleIdle() { }
    void cleanup() { }

    public void OnTriggerEnter(Collider other)
    {
        if (other.GetComponent<Collider>().tag == "RobotCollision")
        {
            hit = true;
        }
    }

    private void moduleMoveForward()
    {
        this.startTime = Time.time;
        transform.position += transform.forward * Time.deltaTime * movementSpeed;
        if (hit)
        {
            this.hit = false;
            this.currentStatus = Status.TERMINATED;
        }
    }

    private void moduleMoveBackward()
    {
        this.startTime = Time.time;
        transform.position -= transform.forward * Time.deltaTime * movementSpeed;
        if (hit)
        {
            this.hit = false;
            this.currentStatus = Status.TERMINATED;
        }
    }
}
