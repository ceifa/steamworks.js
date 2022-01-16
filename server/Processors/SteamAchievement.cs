using Airmiss.Processor;

[ProcessorHub("achievement")]
public class SteamAchievement
{
    [Processor("{name}/activate", Verb.Post)]
    public void Activate([Path("name")] string name)
    {
        new Steamworks.Data.Achievement(name).Trigger(true);
    }
}
