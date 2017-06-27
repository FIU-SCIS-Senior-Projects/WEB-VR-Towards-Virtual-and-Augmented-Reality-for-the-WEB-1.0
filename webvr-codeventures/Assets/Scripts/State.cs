public class State { 
    public string Type { get; set; }
    public string Id { get; set; }

    public State(string id, string type)
    {
        this.Type = type;
        this.Id = id;
    }

    public override string ToString()
    {
        return "State: [" + this.Id + ", " + this.Type + "]";
    }
}
