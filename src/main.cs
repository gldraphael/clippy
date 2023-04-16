const string CLIPPY = @"         
        
         __                 
        /  \        _____________ 
        |  |       /             \
        @  @       | It looks    |
        || ||      | like you    |
        || ||   <--| are very    |
        |\_/|      | bored.      |
        \___/      \_____________/             
          
";

var app = WebApplication.CreateBuilder(args).Build();
app.MapFallback(() => CLIPPY);
app.Run();
