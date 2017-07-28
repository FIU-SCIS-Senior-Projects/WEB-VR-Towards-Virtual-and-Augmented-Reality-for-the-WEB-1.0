public class Transition {
    public string Id { get; set; }
    public State From { get; set; }
    public State To { get; set; }

    public Transition(string id, State from, State to)
    {
        this.Id = id;
        this.From = from;
        this.To = to;
    }
}
