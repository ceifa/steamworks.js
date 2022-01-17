using Airmiss.Processor;

[ProcessorHub("friends")]
public class SteamFriends
{

    [Processor("friends", Verb.Get)]
    public object GetFriends()
    {
        return Steamworks.SteamFriends.GetFriends();
    }

}

