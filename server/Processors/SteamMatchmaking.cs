using Airmiss.Processor;

[ProcessorHub("matchmaking")]
public class SteamMatchmaking
{
    [Processor("createLobbyAsync/{maxMembers}", Verb.Get)]
    public async Task<bool> CreateLobbyAsync([Path("maxMembers")] int maxMembers)
    {    
        try
        {
            var createLobbyOutput = await Steamworks.SteamMatchmaking.CreateLobbyAsync(maxMembers);
            if (!createLobbyOutput.HasValue)
            {
                Console.WriteLine("Lobby created but not correctly instantiated");
                throw new Exception();
            }

            /*
            LobbyPartnerDisconnected = false;
            hostedMultiplayerLobby = createLobbyOutput.Value;
            hostedMultiplayerLobby.SetPublic();
            hostedMultiplayerLobby.SetJoinable(true);
            hostedMultiplayerLobby.SetData(staticDataString, lobbyParameters)  
                    currentLobby = hostedMultiplayerLobby;
            */

            return true;
        }
        catch (Exception exception)
        {
            Console.WriteLine("Failed to create multiplayer lobby");
            Console.WriteLine(exception.ToString());
            return false;
        }

    }

    /*
    void OnLobbyGameCreatedCallback(Lobby lobby, uint ip, ushort port, SteamId steamId)
    {
        AcceptP2P(OpponentSteamId); 
    }

    private void AcceptP2P(long opponentId)
    {
        try
        {
            // For two players to send P2P packets to each other, they each must call this on the other player
            SteamNetworking.AcceptP2PSessionWithUser(opponentId);
        }
        catch
        {
            Debug.Log("Unable to accept P2P Session with user");
        }
    }
    */

}

