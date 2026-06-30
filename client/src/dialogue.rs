//! Dialogue data structures and the full bilingual (DE/EN) dialogue tree.
//!
//! Separated from `story.rs` (which holds the logic) because the text content
//! is large. Every `text_de` / `text_en` pair is the same dialogue in German
//! and English; `state::l()` picks the active language at render time.

use crate::state::{EFF_ENDING, EFF_NONE, EFF_SET_FLAG};

#[derive(Copy, Clone)]
pub struct Choice {
    pub text_de: &'static str,
    pub text_en: &'static str,
    pub next: i32,
    pub effect: i32,
    pub val: i32,
}

pub struct Node {
    pub speaker: &'static str,
    pub color: u32,
    pub text_de: &'static str,
    pub text_en: &'static str,
    pub choices: &'static [Choice],
}

macro_rules! c {
    ($de:expr, $en:expr, $next:expr, $eff:expr, $val:expr) => {
        Choice { text_de: $de, text_en: $en, next: $next, effect: $eff, val: $val }
    };
}

pub static NODES: &[Node] = &[
    // 0: Mathilde greeting
    Node {
        speaker: "Mathilde",
        color: 0xffce7a,
        text_de: "Fremder! Ihr habt Mut, in diesen dunklen Tagen nach Bruma zu kommen. Seid willkommen im Fuchs und Krug. Doch sagt mir - sucht Ihr Schutz, oder sucht Ihr das Höllentor im Osten?",
        text_en: "Stranger! You have courage, coming to Bruma in these dark days. Welcome to the Fox and Mug. But tell me - do you seek shelter, or do you seek the Oblivion gate to the east?",
        choices: &[
            c!("Was ist mit diesem Tor?", "What about this gate?", 1, EFF_NONE, 0),
            c!("Ich will es schließen.", "I want to close it.", 2, EFF_NONE, 0),
            c!("Nur ein Bier, dann gehe ich.", "Just a beer, then I'll go.", -1, EFF_SET_FLAG, crate::state::F_TALKED_MATHILDE as i32),
        ],
    },
    // 1: Mathilde about the gate
    Node {
        speaker: "Mathilde",
        color: 0xffce7a,
        text_de: "Seit drei Nächten brennt ein Tor nach Oblivion im Osten, jenseits des Hains. Daedra kriechen hindurch. Wir haben einen Toten und drei Vermisste. Wache Roderick hält die Südstraße, doch gegen einen Dremora ist Stahl dünn wie Papier.",
        text_en: "For three nights a gate to Oblivion has burned in the east, beyond the grove. Daedra crawl through it. We have one dead and three missing. Guard Roderick holds the south road, but against a Dremora, steel is thin as paper.",
        choices: &[
            c!("Wo kann ich helfen?", "Where can I help?", 2, EFF_NONE, 0),
            c!("Ich sehe mich erst um.", "Let me look around first.", -1, EFF_SET_FLAG, crate::state::F_TALKED_MATHILDE as i32),
        ],
    },
    // 2: Mathilde pointers
    Node {
        speaker: "Mathilde",
        color: 0xffce7a,
        text_de: "Bruder Anselm in der Akatosh-Kapelle betet und flüstert von alten Siegeln. Vielleicht weiß er mehr als eine einfache Wirtin. Und wenn Ihr wirklich vorhabt, das Tor zu betreten - sprecht mit Roderick im Süden.",
        text_en: "Brother Anselm in the Akatosh Chapel prays and whispers of ancient seals. Perhaps he knows more than a simple innkeeper. And if you truly intend to enter the gate - speak with Roderick in the south.",
        choices: &[
            c!("Ich gehe zur Kapelle.", "I'll go to the chapel.", -1, EFF_SET_FLAG, crate::state::F_TALKED_MATHILDE as i32),
            c!("Ich suche Roderick auf.", "I'll find Roderick.", -1, EFF_SET_FLAG, crate::state::F_TALKED_MATHILDE as i32),
        ],
    },
    // 3: Anselm greeting
    Node {
        speaker: "Bruder Anselm",
        color: 0xc8c8d8,
        text_de: "Akatobs Segen sei mit Euch, Wanderer. Ich spüre den Schatten auf Eurer Seele. Sucht Ihr Antworten auf das Tor, das im Osten brennt?",
        text_en: "Akatosh's blessing be upon you, wanderer. I sense a shadow upon your soul. Do you seek answers about the gate that burns in the east?",
        choices: &[
            c!("Ja. Wie kann man es schließen?", "Yes. How can it be closed?", 4, EFF_NONE, 0),
            c!("Wer seid Ihr?", "Who are you?", 5, EFF_NONE, 0),
            c!("Ich muss weiter.", "I must go on.", -1, EFF_NONE, 0),
        ],
    },
    // 4: Anselm reveals the relic
    Node {
        speaker: "Bruder Anselm",
        color: 0xc8c8d8,
        text_de: "Die alten Schriften der Ayleiden sprechen von einem Siegel - dem Herzen von Akatosh, geschmiedet aus Sternenlicht. Es ruht in den Ruinen westlich des Dorfes, jenseits des Teichs. Nur mit ihm lässt sich ein Tor dauerhaft schließen. Doch hütet Euch: Die Mythische Morgenröte sucht es ebenfalls.",
        text_en: "The ancient Ayleid texts speak of a seal - the Heart of Akatosh, forged from starlight. It rests in the ruins west of the village, beyond the pond. Only with it can a gate be closed permanently. But beware: the Mythic Dawn seeks it as well.",
        choices: &[
            c!("Ich hole das Siegel.", "I'll get the seal.", 6, EFF_SET_FLAG, crate::state::F_KNOWS_RELIC as i32),
            c!("Welche Gefahren lauern dort?", "What dangers lurk there?", 7, EFF_NONE, 0),
            c!("Wer ist die Mythische Morgenröte?", "Who is the Mythic Dawn?", 8, EFF_NONE, 0),
        ],
    },
    // 5: Anselm identity
    Node {
        speaker: "Bruder Anselm",
        color: 0xc8c8d8,
        text_de: "Ich bin nur ein demütiger Diener Akatoshs, Hüter dieser kleinen Kapelle. Doch die Schriften kenne ich, soweit ein Sterblicher sie fassen kann.",
        text_en: "I am but a humble servant of Akatosh, keeper of this small chapel. But I know the scriptures, as far as a mortal can grasp them.",
        choices: &[
            c!("Erzählt vom Tor.", "Tell me of the gate.", 4, EFF_NONE, 0),
            c!("Lebt wohl.", "Farewell.", -1, EFF_NONE, 0),
        ],
    },
    // 6: Anselm blesses the quest
    Node {
        speaker: "Bruder Anselm",
        color: 0xc8c8d8,
        text_de: "Geht im Licht Akatoshs. Die Ruinen liegen jenseits des Teichs im Westen - eine versiegelte Tür aus Ayleid-Stein. Und Fremder... vertraut niemandem, der zu viel weiß, seit das Tor brannte.",
        text_en: "Go in Akatosh's light. The ruins lie beyond the pond to the west - a sealed door of Ayleid stone. And stranger... trust no one who knows too much, not since the gate began to burn.",
        choices: &[
            c!("Ich verstehe.", "I understand.", -1, EFF_SET_FLAG, crate::state::F_TALKED_ANSELM as i32),
        ],
    },
    // 7: Anselm on dangers
    Node {
        speaker: "Bruder Anselm",
        color: 0xc8c8d8,
        text_de: "Die Ruinen sind verflucht. Untote Wachen schlafen in den Gängen, und man flüstert von einem Daedra-Fürsten, der das Siegel fordert. Doch schlimmer als die Toten sind die Sterblichen, die ihm dienen.",
        text_en: "The ruins are cursed. Undead guards sleep in the corridors, and there are whispers of a Daedra prince who demands the seal. But worse than the dead are the mortals who serve him.",
        choices: &[
            c!("Ich hole das Siegel.", "I'll get the seal.", 6, EFF_SET_FLAG, crate::state::F_KNOWS_RELIC as i32),
            c!("Wer sind die Mythische Morgenröte?", "Who is the Mythic Dawn?", 8, EFF_NONE, 0),
        ],
    },
    // 8: Anselm on the cult
    Node {
        speaker: "Bruder Anselm",
        color: 0xc8c8d8,
        text_de: "Die Mythische Morgenröte - ein Kult, der den Untergang der Welt herbeisehnt. Sie wollen die Tore öffnen, nicht schließen. Wenn sie von Eurem Vorhaben erfahren, werden sie Euch einholen. Hütet Eure Zunge.",
        text_en: "The Mythic Dawn - a cult that yearns for the world's end. They want to open the gates, not close them. If they learn of your plan, they will come for you. Guard your tongue.",
        choices: &[
            c!("Ich hole das Siegel.", "I'll get the seal.", 6, EFF_SET_FLAG, crate::state::F_KNOWS_RELIC as i32),
            c!("Lebt wohl, Bruder.", "Farewell, brother.", -1, EFF_SET_FLAG, crate::state::F_TALKED_ANSELM as i32),
        ],
    },
    // 9: Roderick greeting
    Node {
        speaker: "Wache Roderick",
        color: 0xb83c34,
        text_de: "Halt, Fremder. Die Südstraße ist gesperrt. Niemand verlässt Bruma, solange das Tor brennt. Was wollt Ihr hier?",
        text_en: "Halt, stranger. The south road is closed. No one leaves Bruma while the gate burns. What do you want here?",
        choices: &[
            c!("Ich will das Tor schließen.", "I want to close the gate.", 10, EFF_NONE, 0),
            c!("Was ist Euer Plan?", "What's your plan?", 11, EFF_NONE, 0),
            c!("Verstanden.", "Understood.", -1, EFF_SET_FLAG, crate::state::F_TALKED_RODERICK as i32),
        ],
    },
    // 10: Roderick on closing
    Node {
        speaker: "Wache Roderick",
        color: 0xb83c34,
        text_de: "Mutig. Oder töricht. Ohne das Siegel der Ayleiden lauft Ihr in den sicheren Tod. Sucht Bruder Anselm in der Kapelle, wenn Ihr noch nicht bei ihm wart.",
        text_en: "Brave. Or foolish. Without the Ayleid seal you walk into certain death. Seek Brother Anselm in the chapel, if you haven't already.",
        choices: &[
            c!("Ich besorge das Siegel.", "I'll get the seal.", -1, EFF_SET_FLAG, crate::state::F_TALKED_RODERICK as i32),
            c!("Ich greife trotzdem an.", "I'll attack anyway.", 12, EFF_NONE, 0),
        ],
    },
    // 11: Roderick's plan
    Node {
        speaker: "Wache Roderick",
        color: 0xb83c34,
        text_de: "Wir stellen die Miliz am Waldrand auf. Drei Männer mit Schwertern, mehr habe ich nicht. Gegen einen Dremora-Lord? Das ist kein Kampf, das ist eine Hinrichtung. Wir brauchen eine andere Waffe.",
        text_en: "We're stationing the militia at the forest edge. Three men with swords, that's all I have. Against a Dremora lord? That's not a fight, that's an execution. We need a different weapon.",
        choices: &[
            c!("Vielleicht habe ich eine.", "Perhaps I have one.", 13, EFF_NONE, 0),
            c!("Ich sehe mich um.", "I'll look around.", -1, EFF_SET_FLAG, crate::state::F_TALKED_RODERICK as i32),
        ],
    },
    // 12: Roderick assault path
    Node {
        speaker: "Wache Roderick",
        color: 0xb83c34,
        text_de: "Dann sterbt Ihr als Held, nicht als Feigling. Ich werde Euren Namen nicht vergessen, Fremder. Möge Akatosh Eure Klinge segnen.",
        text_en: "Then you'll die a hero, not a coward. I won't forget your name, stranger. May Akatosh bless your blade.",
        choices: &[
            c!("Ich gehe zum Tor.", "I'm going to the gate.", -1, EFF_SET_FLAG, crate::state::F_ASSAULT as i32),
        ],
    },
    // 13: Roderick holds the line
    Node {
        speaker: "Wache Roderick",
        color: 0xb83c34,
        text_de: "Das Siegel der Ayleiden? Dann halte ich die Linie, während Ihr das Tor betrett. Ich gebe Euch so viel Zeit, wie drei Leben können. Geht - und verschwendet sie nicht.",
        text_en: "The Ayleid seal? Then I'll hold the line while you enter the gate. I'll give you as much time as three lives can buy. Go - and don't waste it.",
        choices: &[
            c!("Mein Dank, Roderick.", "My thanks, Roderick.", -1, EFF_SET_FLAG, crate::state::F_RODERICK_LINE as i32),
        ],
    },
    // 14: Lyra first meeting
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Still, Wanderer. Ich kenne diesen Blick - die Pilger, die das Tor schließen wollen. Ich bin Lyra. Und ich weiß mehr über Oblivion, als ein Sterblicher wissen sollte.",
        text_en: "Quiet, wanderer. I know that look - the pilgrims who want to close the gate. I am Lyra. And I know more about Oblivion than a mortal should.",
        choices: &[
            c!("Wer seid Ihr wirklich?", "Who are you really?", 15, EFF_NONE, 0),
            c!("Was wollt Ihr von mir?", "What do you want from me?", 16, EFF_NONE, 0),
            c!("Lasst mich in Ruhe.", "Leave me alone.", 17, EFF_SET_FLAG, crate::state::F_REFUSED_LYRA as i32),
        ],
    },
    // 15: Lyra identity
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Nennen wir mich eine... Gelehrte. Ich habe die Tore dieser Provinz studiert, lange bevor sie brannten. Ich weiß, was das Schließen erfordert - und was es kostet. Manche Geheimnisse sind schwerer als Stahl.",
        text_en: "Call me a... scholar. I've studied the gates of this province, long before they burned. I know what closing requires - and what it costs. Some secrets are heavier than steel.",
        choices: &[
            c!("Dann helft mir.", "Then help me.", 16, EFF_NONE, 0),
            c!("Ich vertraue niemandem.", "I trust no one.", 17, EFF_SET_FLAG, crate::state::F_REFUSED_LYRA as i32),
        ],
    },
    // 16: Lyra offer
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Ich kann Euch einen sicheren Weg ans Tor zeigen, und ich kenne Dinge über das Siegel, die Anselm Euch verschwieg. Doch teilen kann man Wissen nur, wenn man es hat. Habt Ihr das Herz von Akatosh?",
        text_en: "I can show you a safe path to the gate, and I know things about the seal that Anselm didn't tell you. But one can only share knowledge if one has it. Do you have the Heart of Akatosh?",
        choices: &[
            c!("Ja, ich trage es.", "Yes, I carry it.", 18, EFF_SET_FLAG, crate::state::F_TOLD_LYRA as i32),
            c!("Nein, noch nicht.", "No, not yet.", 19, EFF_NONE, 0),
            c!("Ich weiß von keinem Siegel.", "I know of no seal.", 20, EFF_SET_FLAG, crate::state::F_REFUSED_LYRA as i32),
        ],
    },
    // 17: Lyra refused
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Wie Ihr wollt. Ich werde hier sein, wenn Ihr es euch anders überlegt. Die Nacht ist lang, und das Tor wartet nicht.",
        text_en: "As you wish. I'll be here if you change your mind. The night is long, and the gate does not wait.",
        choices: &[
            c!("[gehen]", "[leave]", -1, EFF_SET_FLAG, crate::state::F_TALKED_LYRA as i32),
        ],
    },
    // 18: Lyra accepts
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Klug, dass Ihr es mir sagt. Vertrauen ist die erste Währung der Verzweiflung. Kommt, wir gehen gemeinsam zum Tor. Ich führe Euch durch den Hain.",
        text_en: "Clever that you tell me. Trust is the first currency of desperation. Come, let's go to the gate together. I'll guide you through the grove.",
        choices: &[
            c!("Führt mich.", "Lead me.", -1, EFF_SET_FLAG, crate::state::F_ACCEPTED_LYRA as i32),
            c!("Nein, ich gehe allein.", "No, I'll go alone.", -1, EFF_SET_FLAG, crate::state::F_TALKED_LYRA as i32),
        ],
    },
    // 19: Lyra no relic yet
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Dann besorgt es. Ohne das Herz von Akatosh ist das Tor nur ein Grab. Sucht die Ruinen im Westen - doch sagt niemandem, dass ich Euch davon erzählte. Ich warte hier.",
        text_en: "Then get it. Without the Heart of Akatosh the gate is only a grave. Seek the ruins in the west - but tell no one I spoke of it. I'll wait here.",
        choices: &[
            c!("[gehen]", "[leave]", -1, EFF_SET_FLAG, crate::state::F_TALKED_LYRA as i32),
        ],
    },
    // 20: Lyra lied to
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Seid vorsichtig, wen Ihr anlügt, Wanderer. Ich kenne die Wahrheit, ob Ihr sie sprecht oder verschweigt. Geht - und denkt an mich, wenn das Tor vor Euch lodert.",
        text_en: "Be careful who you lie to, wanderer. I know the truth, whether you speak it or not. Go - and think of me when the gate roars before you.",
        choices: &[
            c!("[gehen]", "[leave]", -1, EFF_SET_FLAG, crate::state::F_TALKED_LYRA as i32),
        ],
    },
    // 21: Relic pedestal (no knowledge)
    Node {
        speaker: "",
        color: 0xaaaaaa,
        text_de: "Eine versiegelte Tür aus silbernem Ayleid-Stein. Runen pulsieren schwach, doch sie reagieren nicht auf Euch. Ohne das Wissen um ihre Bedeutung kommt Ihr nicht hinein.",
        text_en: "A sealed door of silver Ayleid stone. Runes pulse faintly, but they don't respond to you. Without the knowledge of their meaning, you cannot enter.",
        choices: &[
            c!("[zurück]", "[back]", -1, EFF_NONE, 0),
        ],
    },
    // 22: Relic pedestal (obtain)
    Node {
        speaker: "",
        color: 0xffe070,
        text_de: "Ihr sprecht die Worte, die Anselm Euch lehrte. Die Runen erlöschen, und die Tür gleitet lautlos auf. In einer Kammer aus Sternenlicht ruht das Herz von Akatosh - ein Splitter reinen Lichts, kalt und schwer in Eurer Hand.",
        text_en: "You speak the words Anselm taught you. The runes fade, and the door slides open silently. In a chamber of starlight rests the Heart of Akatosh - a shard of pure light, cold and heavy in your hand.",
        choices: &[
            c!("Ich nehme das Siegel.", "I take the seal.", -1, EFF_SET_FLAG, crate::state::F_HAS_RELIC as i32),
        ],
    },
    // 23: Gate early
    Node {
        speaker: "",
        color: 0xff7030,
        text_de: "Das Höllentor lodert vor Euch. Hitze schlägt Euch entgegen, und der Gestank von Schwefel erstickt den Atem. Schreie erklingen jenseits der Flammen. Ihr seid noch nicht bereit, es zu betreten.",
        text_en: "The Oblivion gate roars before you. Heat beats against you, and the stench of sulfur chokes your breath. Screams echo beyond the flames. You are not yet ready to enter.",
        choices: &[
            c!("[zurückziehen]", "[retreat]", -1, EFF_SET_FLAG, crate::state::F_GATE_VISITED_EARLY as i32),
        ],
    },
    // 24: Finale - solo with relic
    Node {
        speaker: "",
        color: 0xffe070,
        text_de: "Ihr steht vor dem Tor und haltet das Herz von Akatosh empor. Sternenlicht pulsiert, die Runen auf dem Siegel erwachen zum Leben. Die Flammen zögern - zum ersten Mal seit drei Nächten.",
        text_en: "You stand before the gate and hold up the Heart of Akatosh. Starlight pulses, the runes on the seal come alive. The flames hesitate - for the first time in three nights.",
        choices: &[
            c!("Setzt das Siegel ein.", "Use the seal.", 28, EFF_NONE, 0),
        ],
    },
    // 25: Finale - with Lyra as ally
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Ich stehe zu meinem Wort. Ich halte die Daedra auf, während Ihr das Siegel setzt. Tut es jetzt - ich bin eine Klinge Akavirischen Schwurs, und meine Klinge kennt ihre Sprache.",
        text_en: "I stand by my word. I'll hold off the Daedra while you set the seal. Do it now - I am a blade of Akaviri oath, and my blade knows their language.",
        choices: &[
            c!("Setzt das Siegel ein.", "Use the seal.", 28, EFF_NONE, 0),
        ],
    },
    // 26: Finale - betrayed by Lyra
    Node {
        speaker: "Lyra",
        color: 0x9a6bd0,
        text_de: "Habt Ihr wirklich geglaubt, ich wäre eine Gelehrte? Die Mythische Morgenröte dankt für das Siegel. Es wird uns helfen, die Tore für immer offen zu halten. Bruma brennt heute Nacht - dank Eurer Leichtgläubigkeit.",
        text_en: "Did you really believe I was a scholar? The Mythic Dawn thanks you for the seal. It will help us keep the gates open forever. Bruma burns tonight - thanks to your gullibility.",
        choices: &[
            c!("Ihr habt mich getäuscht!", "You deceived me!", 29, EFF_NONE, 0),
        ],
    },
    // 27: Finale - assault, no relic
    Node {
        speaker: "",
        color: 0xff5050,
        text_de: "Ihr habt kein Siegel. Nur Euren Willen und Euren Stahl. Die Hitze schlägt Euch entgegen, und die Schreie der Daedra erfüllen die Luft. Rodericks Männer halten die Linie hinter Euch - doch diese Linie wird brechen, wenn das Tor nicht fällt.",
        text_en: "You have no seal. Only your will and your steel. The heat beats against you, and the screams of Daedra fill the air. Roderick's men hold the line behind you - but that line will break if the gate doesn't fall.",
        choices: &[
            c!("Stürmt das Tor.", "Storm the gate.", 30, EFF_NONE, 0),
        ],
    },
    // 28: Ending - Held von Bruma
    Node {
        speaker: "",
        color: 0xffe070,
        text_de: "Das Siegel erglüht. Die Tore von Oblivion schließen sich mit einem Donner, der durch ganz Bruma hallt. Der Hain verstummt, die Daedra zerfallen zu Asche. Ihr habt Bruma gerettet. Man wird Euren Namen in Liedern singen - der Fremde, der das Tor schloss.",
        text_en: "The seal glows. The gates of Oblivion close with a thunder that echoes through all of Bruma. The grove falls silent, the Daedra crumble to ash. You have saved Bruma. They will sing your name in songs - the stranger who closed the gate.",
        choices: &[
            c!("[Ende]", "[End]", -1, EFF_ENDING, 1),
        ],
    },
    // 29: Ending - Verraten
    Node {
        speaker: "",
        color: 0x9a6bd0,
        text_de: "Lyra entreißt Euch das Siegel und lacht. Hinter ihr öffnen sich drei weitere Tore, und der Himmel über Bruma färbt sich blutrot. Bruma brennt. Die Mythische Morgenröte hat gesiegt - und Ihr wart ihr Werkzeug.",
        text_en: "Lyra tears the seal from you and laughs. Behind her, three more gates open, and the sky above Bruma turns blood red. Bruma burns. The Mythic Dawn has triumphed - and you were their tool.",
        choices: &[
            c!("[Ende]", "[End]", -1, EFF_ENDING, 2),
        ],
    },
    // 30: Ending - Opfer
    Node {
        speaker: "",
        color: 0xb83c34,
        text_de: "Ihr stürmt in die Flammen. Eure Schreie hallen durch die Nacht, doch das Tor wankt unter Eurer Verzweiflung. Es schließt sich - und verschlingt Euch. Bruma lebt. Ihr kehrt nicht zurück. Man wird sagen, ein Held sei hier gestorben.",
        text_en: "You storm into the flames. Your screams echo through the night, but the gate falters under your desperation. It closes - and devours you. Bruma lives. You do not return. They will say a hero died here.",
        choices: &[
            c!("[Ende]", "[End]", -1, EFF_ENDING, 3),
        ],
    },
    // 31: Bauer Konrad greeting (tavern interior)
    Node {
        speaker: "Bauer Konrad",
        color: 0xd0a050,
        text_de: "Noch ein Bier, Mathilde! ... Huch, ein Fremder. Setzt Euch zu mir. Ihr seht aus, als lastet etwas Schweres auf Euch. Das Höllentor im Osten? Ja, wir alle hören es nachts. Die Schreie.",
        text_en: "Another beer, Mathilde! ... Oh, a stranger. Sit with me. You look like something heavy weighs on you. The Oblivion gate in the east? Yes, we all hear it at night. The screams.",
        choices: &[
            c!("Was wisst Ihr vom Tor?", "What do you know of the gate?", 32, EFF_NONE, 0),
            c!("Kennt Ihr die Ruinen im Westen?", "Do you know the ruins in the west?", 33, EFF_NONE, 0),
            c!("Lebt wohl.", "Farewell.", -1, EFF_SET_FLAG, crate::state::F_TALKED_KONRAD as i32),
        ],
    },
    // 32: Konrad on the gate
    Node {
        speaker: "Bauer Konrad",
        color: 0xd0a050,
        text_de: "Geschichten aus meiner Jugend. Mein Großvater war Jäger im Hain und sah einmal ein Tor sich öffnen - ein Spalt aus Feuer und Nacht. Er sagte, nur Sternenlicht könne es schließen. Aber wer glaubt schon alten Jägern?",
        text_en: "Stories from my youth. My grandfather was a hunter in the grove and once saw a gate open - a rift of fire and night. He said only starlight could close it. But who believes old hunters?",
        choices: &[
            c!("Sternenlicht...", "Starlight...", 34, EFF_NONE, 0),
            c!("[zurück]", "[back]", 31, EFF_NONE, 0),
        ],
    },
    // 33: Konrad on the ruins
    Node {
        speaker: "Bauer Konrad",
        color: 0xd0a050,
        text_de: "Die Ayleid-Ruinen? Ja, westlich des Teichs. Mein Großvater jagte dort, doch er betrat sie nie. Er sagte, die Toten schlafen dort, und sie wachen auf, wenn man sie stört. Ich würde nicht hingehen, Fremder. Nicht ohne Grund.",
        text_en: "The Ayleid ruins? Yes, west of the pond. My grandfather hunted there, but he never entered them. He said the dead sleep there, and they wake if you disturb them. I wouldn't go there, stranger. Not without reason.",
        choices: &[
            c!("[zurück]", "[back]", 31, EFF_NONE, 0),
        ],
    },
    // 34: Konrad on the seal
    Node {
        speaker: "Bauer Konrad",
        color: 0xd0a050,
        text_de: "Ja, Sternenlicht. Es gibt ein Siegel - das Herz von Akatosh, nannte er es. Geschmiedet aus einem Splitter der Sterne. Ob es wahr ist? Ich weiß es nicht. Aber Bruder Anselm in der Kapelle kennt die alten Schriften. Fragt ihn.",
        text_en: "Yes, starlight. There's a seal - the Heart of Akatosh, he called it. Forged from a shard of the stars. Is it true? I don't know. But Brother Anselm in the chapel knows the ancient texts. Ask him.",
        choices: &[
            c!("Ich werde ihn fragen.", "I'll ask him.", -1, EFF_SET_FLAG, crate::state::F_TALKED_KONRAD as i32),
            c!("[zurück]", "[back]", 31, EFF_NONE, 0),
        ],
    },
    // 35: Schwester Mildred greeting (chapel interior)
    Node {
        speaker: "Schwester Mildred",
        color: 0xe0d0a0,
        text_de: "Akatosh sei mit Euch. Ich bete für die Seelen, die das Tor verschlungen hat. Sucht Ihr Trost, Fremder, oder sucht Ihr etwas anderes?",
        text_en: "Akatosh be with you. I pray for the souls the gate has devoured. Do you seek comfort, stranger, or do you seek something else?",
        choices: &[
            c!("Ich suche eine Waffe gegen das Tor.", "I seek a weapon against the gate.", 36, EFF_NONE, 0),
            c!("Für wen betet Ihr?", "For whom do you pray?", 37, EFF_NONE, 0),
            c!("Lebt wohl.", "Farewell.", -1, EFF_SET_FLAG, crate::state::F_TALKED_MILDRED as i32),
        ],
    },
    // 36: Mildred on the seal
    Node {
        speaker: "Schwester Mildred",
        color: 0xe0d0a0,
        text_de: "Eine Waffe? Bruder Anselm spricht von einem alten Siegel - dem Herzen von Akatosh. Er betet oft allein in der Krypta, sucht nach Antworten in den Schriften. Sprecht mit ihm, wenn er zurückkehrt.",
        text_en: "A weapon? Brother Anselm speaks of an ancient seal - the Heart of Akatosh. He often prays alone in the crypt, searching for answers in the scriptures. Speak with him when he returns.",
        choices: &[
            c!("[zurück]", "[back]", 35, EFF_NONE, 0),
        ],
    },
    // 37: Mildred on the cult
    Node {
        speaker: "Schwester Mildred",
        color: 0xe0d0a0,
        text_de: "Für alle. Für die Toten, die das Tor genommen hat. Für die Lebenden, die es noch nehmen wird. Und für jene, die den Mut haben, es zu schließen. Seid vorsichtig, Fremder. Die Mythische Morgenröte hat Späher im Dorf.",
        text_en: "For everyone. For the dead the gate has taken. For the living it will yet take. And for those who have the courage to close it. Be careful, stranger. The Mythic Dawn has scouts in the village.",
        choices: &[
            c!("Späher? Hier?", "Scouts? Here?", 38, EFF_NONE, 0),
            c!("[zurück]", "[back]", 35, EFF_NONE, 0),
        ],
    },
    // 38: Mildred on the spies
    Node {
        speaker: "Schwester Mildred",
        color: 0xe0d0a0,
        text_de: "Ich habe sie gesehen - Gestalten im Wald bei Nacht. Sie flüstern zu den Toten und den Daedra. Ich traue niemandem mehr, nicht einmal den Nachbarn. Bewahrt Euer Geheimnis, Fremder. Wer immer behauptet, Euch helfen zu wollen, könnte einer von ihnen sein.",
        text_en: "I've seen them - figures in the forest at night. They whisper to the dead and the Daedra. I trust no one anymore, not even my neighbors. Keep your secret, stranger. Whoever claims to help you could be one of them.",
        choices: &[
            c!("[zurück]", "[back]", 35, EFF_NONE, 0),
        ],
    },
    // 39: Jaeger Erik greeting (house 1 interior)
    Node {
        speaker: "Jaeger Erik",
        color: 0x8aaa60,
        text_de: "Ah, ein Besucher. Gebt acht vor dem Chaos - ich hab gerade meinen Bogen gereinigt. Erik ist der Name. Ich jage die Waelder um Bruma, oder tat es, bevor das Tor oeffnete. Jetzt sind die Waelder voll von diesen... Dingen.",
        text_en: "Ah, a visitor. Mind the clutter - I was just cleaning my bow. Erik's the name. I hunt the forests around Bruma, or I did, before the gate opened. Now the woods are full of those... things.",
        choices: &[
            c!("Was fuer Dinge?", "What things?", 40, EFF_NONE, 0),
            c!("Kennt Ihr die Ruinen?", "Do you know the ruins?", 41, EFF_NONE, 0),
            c!("Lebt wohl.", "Farewell.", -1, EFF_SET_FLAG, crate::state::F_TALKED_ERIK as i32),
        ],
    },
    // 40: Erik on creatures
    Node {
        speaker: "Jaeger Erik",
        color: 0x8aaa60,
        text_de: "Daedra. Ich hab sie nachts im Hain gesehen - spinnenartige Dinge und etwas Groesseres, mit einer Klinge aus Feuer. Meine Pfeile prallen von ihren Haeuten ab wie von Stein. Was immer Ihr vorhabt, Fremder - geht nicht allein.",
        text_en: "Daedra. I've seen them in the grove at night - spider-like things and something larger, with a blade of fire. My arrows bounce off their hides like off stone. Whatever you're planning, stranger - don't go alone.",
        choices: &[
            c!("[zurueck]", "[back]", 39, EFF_NONE, 0),
        ],
    },
    // 41: Erik on ruins
    Node {
        speaker: "Jaeger Erik",
        color: 0x8aaa60,
        text_de: "Die Ayleid-Ruinen westlich des Teichs? Ich hab dort jahrelang gejagt. Seltsame Lichter nachts, und eine Kaelte, die nicht natuerlich ist. Ich fand einmal Spuren - gestiefelte Fuesse, keine Daedra. Jemand besucht diese Ruinen. Kultisten, vielleicht.",
        text_en: "The Ayleid ruins west of the pond? I've hunted near them for years. Strange lights there at night, and a cold that isn't natural. I found tracks once - booted feet, not Daedra. Someone visits those ruins. Cultists, maybe.",
        choices: &[
            c!("[zurueck]", "[back]", 39, EFF_NONE, 0),
        ],
    },
    // 42: Alchemistin Sora greeting (house 2 interior)
    Node {
        speaker: "Alchemistin Sora",
        color: 0x60c0a0,
        text_de: "Oh! Ein Besucher. Bitte, fasst die Flaeschchen nicht an - einige von ihnen sind... launisch. Ich bin Sora. Ich studiere die Eigenschaften von Ayleid-Kristallen und Daedra-Essenzen. Ein gefaehrliches Hobby in diesen Tagen.",
        text_en: "Oh! A visitor. Please, don't touch the flasks - some of them are... temperamental. I'm Sora. I study the properties of Ayleid crystals and Daedric essences. A dangerous hobby these days.",
        choices: &[
            c!("Was habt Ihr gelernt?", "What have you learned?", 43, EFF_NONE, 0),
            c!("Kennt Ihr den Kult?", "Do you know the cult?", 44, EFF_NONE, 0),
            c!("Lebt wohl.", "Farewell.", -1, EFF_SET_FLAG, crate::state::F_TALKED_SORA as i32),
        ],
    },
    // 43: Sora on the seal
    Node {
        speaker: "Alchemistin Sora",
        color: 0x60c0a0,
        text_de: "Die Ayleiden banden Sternenlicht in Kristalle - das weiss ich. Ihre Siegel konnten Risse zwischen Welten schliessen. Wenn es ein Siegel gibt, das das Tor schliessen kann, dann in den westlichen Ruinen. Eine alte Macht, aber die einzige, die gegen Oblivion wirkt.",
        text_en: "The Ayleids bound starlight into crystals - that much I know. Their seals could close rifts between worlds. If there's a seal that can shut that gate, it would be in the western ruins. An old power, but the only one that works against Oblivion.",
        choices: &[
            c!("[zurueck]", "[back]", 42, EFF_NONE, 0),
        ],
    },
    // 44: Sora on the cult
    Node {
        speaker: "Alchemistin Sora",
        color: 0x60c0a0,
        text_de: "Die Mythische Morgenroete? Ja, ich bin... ihrer Arbeit begegnet. Daedra-Essenzen im Wald, sorgfaeltig destilliert. Jemand mit Wissen hilft ihnen. Seid vorsichtig bei jedem, der zu viele Fragen ueber das Tor oder die Ruinen stellt - sie berichten zurueck, da bin ich mir sicher.",
        text_en: "The Mythic Dawn? Yes, I've... encountered their work. Daedric essences in the forest, carefully distilled. Someone with knowledge is helping them. Be wary of anyone who asks too many questions about the gate or the ruins - they report back, I'm sure of it.",
        choices: &[
            c!("[zurueck]", "[back]", 42, EFF_NONE, 0),
        ],
    },
];
