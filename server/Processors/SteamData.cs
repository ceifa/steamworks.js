using Airmiss.Processor;

[ProcessorHub("data")]
public class SteamData
{

    [Processor("inviteFriend/{friendId}", Verb.Get)]
    public static bool InviteFriend([Path("friendId")] ulong friendId)
    {
        var instanceLobby = new Steamworks.Data.Lobby();
        return instanceLobby.InviteFriend(friendId);
    }


    /*
    [ProcessorHub("data/lobby")]
    public static class Lobby  
    {
        [Processor("data/lobby/inviteFriend", Verb.Get)]
        public static bool InviteFriend([Path("friendId")] ulong friendId)
        {
            var instanceLobby = new Steamworks.Data.Lobby();          
            return instanceLobby.InviteFriend(friendId);
        }

        [Processor("data/lobby/setFriendsOnly", Verb.Get)]
        public static bool SetFriendsOnly()
        {
            var instanceLobby = new Steamworks.Data.Lobby();
            return instanceLobby.SetFriendsOnly();
        }
    }

    */
}
