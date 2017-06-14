using System;
using System.Collections.Generic;
using System.Linq;

public class StateMachine {
    private State _initialState;
    private State _currentState;
    private readonly List<State> _states;
    private readonly List<Transition> _transitions;
    private int _step = 0;

    public StateMachine()
    {
        this._initialState = null;
        this._currentState = null;
        this._states = new List<State>();
        this._transitions = new List<Transition>();
    }

    public StateMachine(State initial, List<State> states, List<Transition> transitions)
    {
        this._initialState = initial;
        this._states = states;
        this._transitions = transitions;
        this._currentState = this._initialState;
    }

    public void AddState(State state)
    {
        this._states.Add(state);
    }

    public void SetInitialState(State state)
    {
        this._initialState = state;
        this._currentState = this._initialState;
    }

    private void AddTransitions(String id, State from, State to)
    {
        this._transitions.Add(new Transition(id, from, to));
    }

    public void AddTransitions(String id, String source, String target)
    {
        if (this._states.Exists(x => x.Id == source) && this._states.Exists(x => x.Id == target))
        {
            AddTransitions(id, this._states.Find(x => x.Id == source), this._states.Find(x => x.Id == target));
        }
        else
        {
            Console.WriteLine("target or source not found.");
        }
    }

    public State Next()
    {
        var transitionsFromCurrent = this._transitions.Where(x => x.From == this._currentState).ToList();
        this._currentState = transitionsFromCurrent[0].To;
        Console.WriteLine(this._currentState.Type);
        return this._currentState;
    }
}
