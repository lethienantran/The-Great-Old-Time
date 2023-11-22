<h1>The Great Old Time</h1>
<h2>Author: Le Thien An Tran</h2>
<img src="https://github.com/lethienantran/The-Great-Old-Time/assets/114910291/1f1fd026-818d-433a-b6ca-60c6455521cf"/>
<h3>Description</h3>
<p>This is an indie, simple text-based game programming in Rust. The main goal of this game is to defeat 120 bosses. The player might need to kill the bosses twice or even more than twice. Player will have their own stats and buffs in order to fight against those bosses.</p>
<p>The player must be use the advance of buffs and levels so that they can win against the boss. Each boss's has different shield, you have to destroy their shield first to start killing them. The boss will get stronger after every fight. The player will be able to get gold by fighting with bosses and spend those golds for buffs and healing. There are 3 types of buff: 
  <ul>
    <li>Red Buff (Give you bonus attack).</li>
    <li>Blue Buff (Give you bonus defense).</li>
    <li>Bruiser Buff (Give you bonus max health).</li>
  </ul>
</p>
<p>In order to make an option, you just need to type number. For example: if there is Option 1: Start New Game, then you just need to input 1. </p>
<p>The player has a choice to keep continue playing even when they won the game. Don't worry, the game will be saved locally in your folder once you run the program and play.</p>
<h3>How to run the program?</h3>
<p>Before perform all the steps below, you must ensure that you have installed Rust on your machine. After that, turn on your terminal and let's get started: </p>
<ol>
  <li>Run Command: <code>git clone https://github.com/lethienantran/The-Great-Old-Time.git</code></li>
  <li>Locate to the local repository that you have cloned.</li>
  <li>Run Command: <code>cargo build</code></li>
  <li>Run Command: <code>cargo run</code></li>
</ol>
<p>Easy, right? You're good to go and enjoy the game!</p>
<h3>Challenges</h3>
<p>Rust is quite new to me. I am in a learning process in programming in Rust. First of all, I have tried a lot of ways and read a lot of documents to put struct in different files, but somehow I can't do it (Maybe I haven't research about it much). Secondly, I tried to do in an object-oriented way, so that the code is more organized and meaningful, however, it is a challenge for me to do in Rust. Last but not least, finish this game in 2 hours is also really a challenge to me. Haha! However, the game turns out working and I really like it. It helps me understand more about Rust and the concept of low level language.</p>
<h3>Learning/Outcome</h3>
<p>
  <ol>
    <li>
      <strong>Read and Parse JSON file in Rust to get data. Handling local persistence.</strong>
      <p>
      Implemented JSON file parsing in Rust, acquiring essential game data. Additionally, incorporated local persistence for saving and loading game states, enhancing the player's personalized gaming experience.
      </p>
    </li>
    <li>
     <strong>Better knowledge of file I/O in Rust.</strong>
    <p>
      Deepened understanding of file input/output (I/O) operations in Rust. This skill is crucial for managing external data, such as saving game progress or reading configuration files, therefore, I'm glad that I learned it.
    </p>
  </li>

  <li>
    <strong>Understand more about game design concepts such as player progression, boss difficulty scaling, in-game economy (gold and buffs), and user interaction.</strong>
    <p>
      Gained insights into fundamental game design principles, including player progression, dynamic boss difficulty scaling, in-game economy management (gold and buffs), and user-friendly interaction. Balancing these elements ensured an engaging and challenging gameplay experience.
    </p>
  </li>

  <li>
    <strong>Object-Oriented Challenges and Rust Concepts.</strong>
    <p>
      Faced challenges while attempting an object-oriented design in Rust. Although, I have not achieved this, but it worth my research for exploring the structuring of code for better organization and meaning. Overcoming these challenges deepened understanding of Rust's ownership system, borrowing, and lifetimes.
    </p>
  </li>

  <li>
    <strong>Time Management and Rapid Development.</strong>
    <p>
      Completed the game within a tight timeframe, showcasing effective time management and quick decision-making. The experience honed skills in prioritizing tasks, making efficient coding decisions, and delivering a functional project under constraints.
    </p>
  </li>

  <li>
    <strong>Appreciation for Low-Level Language Concepts.</strong>
    <p>
      I have developed an appreciation for low-level language concepts inherent in Rust. This included understanding Rust's unique features such as ownership, borrowing, and lifetimes, which contribute to safety guarantees without sacrificing performance.
    </p>
  </li>
  </ol>
</p>
<h3>Screenshots</h3>
<h4>Welcome Screen & New Game Screen</h4>
<img src="https://github.com/lethienantran/The-Great-Old-Time/assets/114910291/02ce281e-fa7c-4335-a2e9-320ac91af589"/>
<h4>Instruction Screen</h4>
<img src="https://github.com/lethienantran/The-Great-Old-Time/assets/114910291/10b8b853-de42-4b3b-b196-fac34d1538df"/>
<h4>Fight Boss Screen</h4>
<img src="https://github.com/lethienantran/The-Great-Old-Time/assets/114910291/7e30517f-2a5a-4bd2-bb64-b3a71d69edaa"/>
<h4>Boss Defeated & Level Up Screen</h4>
<img src="https://github.com/lethienantran/The-Great-Old-Time/assets/114910291/ab4f3d56-ee44-4289-92c4-4439f8e325ae"/>
<h4>Get Player's Stats & Shop Screen</h4>
<img src="https://github.com/lethienantran/The-Great-Old-Time/assets/114910291/63074305-62e3-4c6e-aef1-9c8b4484f153"/>
