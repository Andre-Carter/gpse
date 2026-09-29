use std::fmt;

#[derive(Debug)]
pub struct Verse {
    pub content: &'static str,
}

impl fmt::Display for Verse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

pub fn lookup(book: &str, chapter: u8, verse: u8) -> Option<Verse> {
    match (book, chapter, verse) {
        ("genesis", 1, 1) => Some(Verse {
            content: "In the beginning God created the heaven and the earth.",
        }),

        ("genesis", 1, 2) => Some(Verse {
            content: "And the earth was without forme, and voyd, and darkenesse was vpon the face of the deepe: and the Spirit of God mooued vpon the face of the waters.",
        }),

        ("genesis", 1, 3) => Some(Verse {
            content: "And God said, Let there be light: and there was light.",
        }),

        ("genesis", 1, 4) => Some(Verse {
            content: "And God saw the light, that it was good: and God diuided the light from the darkenesse.",
        }),

        ("genesis", 1, 5) => Some(Verse {
            content: "And God called the light, Day, and the darknesse he called Night: and the euening and the morning were the first day.",
        }),

        ("genesis", 1, 6) => Some(Verse {
            content: "And God said, Let there be a firmament in the midst of the waters: and let it diuide the waters from the waters.",
        }),

        ("genesis", 1, 7) => Some(Verse {
            content: "And God made the firmament; and diuided the waters, which were vnder the firmament, from the waters, which were aboue the firmament: and it was so.",
        }),

        ("genesis", 1, 8) => Some(Verse {
            content: "And God called the firmament, Heauen: and the euening and the morning were the second day.",
        }),

        ("genesis", 1, 9) => Some(Verse {
            content: "And God said, Let the waters vnder the heauen be gathered together vnto one place, and let the dry land appeare: and it was so.",
        }),

        ("genesis", 1, 10) => Some(Verse {
            content: "And God called the drie land, Earth, and the gathering together of the waters called hee, Seas: and God saw that it was good.",
        }),

        ("genesis", 1, 11) => Some(Verse {
            content: "And God said, Let the Earth bring foorth grasse, the herbe yeelding seed, and the fruit tree, yeelding fruit after his kinde, whose seed is in it selfe, vpon the earth: and it was so.",
        }),

        ("genesis", 1, 12) => Some(Verse {
            content: "And the earth brought foorth grasse, and herbe yeelding seed after his kinde, and the tree yeelding fruit, whose seed was in it selfe, after his kinde: and God saw that it was good.",
        }),

        ("genesis", 1, 13) => Some(Verse {
            content: "And the euening and the morning were the third day.",
        }),

        ("genesis", 1, 14) => Some(Verse {
            content: "And God said, Let there bee lights in the firmament of the heauen, to diuide the day from the night: and let them be for signes and for seasons, and for dayes and yeeres.",
        }),

        ("genesis", 1, 15) => Some(Verse {
            content: "And let them be for lights in the firmament of the heauen, to giue light vpon the earth: and it was so.",
        }),

        ("genesis", 1, 16) => Some(Verse {
            content: "And God made two great lights: the greater light to rule the day, and the lesser light to rule the night: he made the starres also.",
        }),

        ("genesis", 1, 17) => Some(Verse {
            content: "And God set them in the firmament of the heauen, to giue light vpon the earth:",
        }),

        ("genesis", 1, 18) => Some(Verse {
            content: "And to rule ouer the day, and ouer the night, and to diuide the light from the darkenesse: and God saw that it was good.",
        }),

        ("genesis", 1, 19) => Some(Verse {
            content: "And the euening and the morning were the fourth day.",
        }),

        ("genesis", 1, 20) => Some(Verse {
            content: "And God said, Let the waters bring foorth aboundantly the mouing creature that hath life, and foule that may flie aboue the earth in the open firmament of heauen.",
        }),

        ("genesis", 1, 21) => Some(Verse {
            content: "And God created great whales, and euery liuing creature that moueth, which the waters brought forth aboundantly after their kinde, and euery winged foule after his kinde: and God saw that it was good.",
        }),

        ("genesis", 1, 22) => Some(Verse {
            content: "And God blessed them, saying, Be fruitfull, and multiply, and fill the waters in the Seas, and let foule multiply in the earth.",
        }),

        ("genesis", 1, 23) => Some(Verse {
            content: "And the euening and the morning were the fift day.",
        }),

        ("genesis", 1, 24) => Some(Verse {
            content: "And God said, Let the earth bring forth the liuing creature after his kinde, cattell, and creeping thing, and beast of the earth after his kinde: and it was so.",
        }),

        ("genesis", 1, 25) => Some(Verse {
            content: "And God made the beast of the earth after his kinde, and cattell after their kinde, and euery thing that creepeth vpon the earth, after his kinde: and God saw that it was good.",
        }),

        ("genesis", 1, 26) => Some(Verse {
            content: "And God said, Let vs make man in our Image, after our likenesse: and let them haue dominion ouer the fish of the sea, and ouer the foule of the aire, and ouer the cattell, and ouer all the earth, and ouer euery creeping thing that creepeth vpon the earth.",
        }),

        ("genesis", 1, 27) => Some(Verse {
            content: "So God created man in his owne Image, in the Image of God created hee him; male and female created hee them.",
        }),

        ("genesis", 1, 28) => Some(Verse {
            content: "And God blessed them, and God said vnto them, Be fruitfull, and multiply, and replenish the earth, and subdue it, and haue dominion ouer the fish of the sea, and ouer the foule of the aire, and ouer euery liuing thing that mooueth vpon the earth.",
        }),

        ("genesis", 1, 29) => Some(Verse {
            content: "And God said, Behold, I haue giuen you euery herbe bearing seede, which is vpon the face of all the earth, and euery tree, in the which is the fruit of a tree yeelding seed, to you it shall be for meat:",
        }),

        ("genesis", 1, 30) => Some(Verse {
            content: "And to euery beast of the earth, and to euery foule of the aire, and to euery thing that creepeth vpon the earth, wherein there is life, I haue giuen euery greene herbe for meat: and it was so.",
        }),

        ("genesis", 1, 31) => Some(Verse {
            content: "And God saw euery thing that hee had made: and behold, it was very good. And the euening and the morning were the sixth day.",
        }),

        ("genesis", 2, 1) => Some(Verse {
            content: "Thus the heauens and the earth were finished, and all the hoste of them.",
        }),

        ("genesis", 2, 2) => Some(Verse {
            content: "And on the seuenth day God ended his worke, which hee had made: And he rested on the seuenth day from all his worke, which he had made.",
        }),

        ("genesis", 2, 3) => Some(Verse {
            content: "And God blessed the seuenth day, and sanctified it: because that in it he had rested from all his worke, which God created and made.",
        }),

        ("genesis", 2, 4) => Some(Verse {
            content: "These are the generations of the heauens, & of the earth, when they were created; in the day that the LORD God made the earth, and the heauens,",
        }),

        ("genesis", 2, 5) => Some(Verse {
            content: "And euery plant of the field, before it was in the earth, and euery herbe of the field, before it grew: for the LORD God had not caused it to raine vpon the earth, and there was not a man to till the ground.",
        }),

        ("genesis", 2, 6) => Some(Verse {
            content: "But there went vp a mist from the earth, and watered the whole face of the ground.",
        }),

        ("genesis", 2, 7) => Some(Verse {
            content: "And the LORD God formed man of the dust of the ground, & breathed into his nostrils the breath of life; and man became a liuing soule.",
        }),

        ("genesis", 2, 8) => Some(Verse {
            content: "And the LORD God planted a garden Eastward in Eden; and there he put the man whom he had formed.",
        }),

        ("genesis", 2, 9) => Some(Verse {
            content: "And out of the ground made the LORD God to grow euery tree that is pleasant to the sight, and good for food: the tree of life also in the midst of the garden, and the tree of knowledge of good and euill.",
        }),

        ("genesis", 2, 10) => Some(Verse {
            content: "And a riuer went out of Eden to water the garden, and from thence it was parted, and became into foure heads.",
        }),

        ("genesis", 2, 11) => Some(Verse {
            content: "The name of the first is Pison: that is it which compasseth the whole land of Hauilah, where there is gold.",
        }),

        ("genesis", 2, 12) => Some(Verse {
            content: "The name of the first is Pison: that is it which compasseth the whole land of Hauilah, where there is gold.",
        }),

        ("genesis", 2, 13) => Some(Verse {
            content: "And the name of the second riuer is Gihon: the same is it that compasseth the whole land of Ethiopia.",
        }),

        ("genesis", 2, 14) => Some(Verse {
            content: "And the name of the third riuer is Hiddekel: that is it which goeth toward the East of Assyria: and the fourth riuer is Euphrates.",
        }),

        ("genesis", 2, 15) => Some(Verse {
            content: "And the LORD God tooke the man, and put him into the garden of Eden, to dresse it, and to keepe it.",
        }),

        ("genesis", 2, 16) => Some(Verse {
            content: "And the LORD God commanded the man, saying, Of euery tree of the garden thou mayest freely eate.",
        }),

        ("genesis", 2, 17) => Some(Verse {
            content: "But of the tree of the knowledge of good and euill, thou shalt not eate of it: for in the day that thou eatest thereof, thou shalt surely die.",
        }),

        ("genesis", 2, 18) => Some(Verse {
            content: "And the LORD God said, It is not good that the man should be alone: I will make him an helpe meet for him.",
        }),

        ("genesis", 2, 19) => Some(Verse {
            content: "And out of þe ground the LORD God formed euery beast of the field, and euery foule of the aire, and brought them vnto Adam, to see what he would call them: and whatsoeuer Adam called euery liuing creature, that was the name thereof.",
        }),

        ("genesis", 2, 20) => Some(Verse {
            content: "And Adam gaue names to all cattell, and to the foule of the aire, and to euery beast of the fielde: but for Adam there was not found an helpe meete for him.",
        }),

        ("genesis", 2, 21) => Some(Verse {
            content: "And the LORD God caused a deepe sleepe to fall vpon Adam, and hee slept; and he tooke one of his ribs, and closed vp the flesh in stead thereof.",
        }),

        ("genesis", 2, 22) => Some(Verse {
            content: "And the rib which the LORD God had taken from man, made hee a woman, & brought her vnto the man.",
        }),

        ("genesis", 2, 23) => Some(Verse {
            content: "And Adam said, This is now bone of my bones, and flesh of my flesh: she shalbe called woman, because shee was taken out of man.",
        }),

        ("genesis", 2, 24) => Some(Verse {
            content: "Therefore shall a man leaue his father and his mother, and shall cleaue vnto his wife: and they shalbe one flesh.",
        }),

        ("genesis", 2, 25) => Some(Verse {
            content: "And they were both naked, the man & his wife, and were not ashamed.",
        }),

        ("genesis", 3, 1) => Some(Verse {
            content: "Now the serpent was more subtill then any beast of the field, which the LORD God had made, and he said vnto the woman, Yea, hath God said, Ye shall not eat of euery tree of the garden?",
        }),

        ("genesis", 3, 2) => Some(Verse {
            content: "And the woman said vnto the serpent, Wee may eate of the fruite of the trees of the garden:",
        }),

        ("genesis", 3, 3) => Some(Verse {
            content: "But of the fruit of the tree, which is in the midst of the garden, God hath said, Ye shal not eate of it, neither shall ye touch it, lest ye die.",
        }),

        ("genesis", 3, 4) => Some(Verse {
            content: "And the Serpent said vnto the woman, Ye shall not surely die.",
        }),

        ("genesis", 3, 5) => Some(Verse {
            content: "For God doeth know, that in the day ye eate thereof, then your eyes shalbee opened: and yee shall bee as Gods, knowing good and euill.",
        }),

        ("genesis", 3, 6) => Some(Verse {
            content: "And when the woman saw, that the tree was good for food, and that it was pleasant to the eyes, and a tree to be desired to make one wise, she tooke of the fruit thereof, and did eate, and gaue also vnto her husband with her, and hee did eate.",
        }),

        ("genesis", 3, 7) => Some(Verse {
            content: "And the eyes of them both were opened, & they knew that they were naked, and they sewed figge leaues together, and made themselues aprons.",
        }),

        ("genesis", 3, 8) => Some(Verse {
            content: "And they heard the voyce of the LORD God, walking in the garden in the coole of the day: and Adam and his wife hid themselues from the presence of the LORD God, amongst the trees of the garden.",
        }),

        ("genesis", 3, 9) => Some(Verse {
            content: "And the LORD God called vnto Adam, and said vnto him, Where art thou?",
        }),

        ("genesis", 3, 10) => Some(Verse {
            content: "And he said, I heard thy voice in the garden: and I was afraid, because I was naked, and I hid my selfe.",
        }),

        ("genesis", 3, 11) => Some(Verse {
            content: "And he said, Who told thee, that thou wast naked? Hast thou eaten of the tree, whereof I commanded thee, that thou shouldest not eate?",
        }),

        ("genesis", 3, 12) => Some(Verse {
            content: "And the man said, The woman whom thou gauest to be with mee, shee gaue me of the tree, and I did eate.",
        }),

        ("genesis", 3, 13) => Some(Verse {
            content: "And the LORD God said vnto the woman, What is this that thou hast done? And the woman said, The Serpent beguiled me, and I did eate.",
        }),

        ("genesis", 3, 14) => Some(Verse {
            content: "And the LORD God said vnto the Serpent, Because thou hast done this, thou art cursed aboue all cattel, and aboue euery beast of the field: vpon thy belly shalt thou goe, and dust shalt thou eate, all the dayes of thy life.",
        }),

        ("genesis", 3, 15) => Some(Verse {
            content: "And I will put enmitie betweene thee and the woman, and betweene thy seed and her seed: it shal bruise thy head, and thou shalt bruise his heele.",
        }),

        ("genesis", 3, 16) => Some(Verse {
            content: "Unto the woman he said, I will greatly multiply thy sorowe and thy conception. In sorow thou shalt bring forth children: and thy desire shall be to thy husband, and hee shall rule ouer thee.",
        }),

        ("genesis", 3, 17) => Some(Verse {
            content: "And vnto Adam he said, Because thou hast hearkened vnto the voyce of thy wife, and hast eaten of the tree, of which I commaunded thee, saying, Thou shalt not eate of it: cursed is the ground for thy sake: in sorow shalt thou eate of it all the dayes of thy life.",
        }),

        ("genesis", 3, 18) => Some(Verse {
            content: "Thornes also and thistles shall it bring forth to thee: and thou shalt eate the herbe of the field.",
        }),

        ("genesis", 3, 19) => Some(Verse {
            content: "In the sweate of thy face shalt thou eate bread, till thou returne vnto the ground: for out of it wast thou taken, for dust thou art, and vnto dust shalt thou returne.",
        }),

        ("genesis", 3, 20) => Some(Verse {
            content: "And Adam called his wiues name Eue, because she was the mother of all liuing.",
        }),

        ("genesis", 3, 21) => Some(Verse {
            content: "Unto Adam also, and to his wife, did the LORD God make coates of skinnes, and cloathed them.",
        }),

        ("genesis", 3, 22) => Some(Verse {
            content: "And the LORD God said, Behold, the man is become as one of vs, to know good & euill. And now lest hee put foorth his hand, and take also of the tree of life, and eate and liue for euer:",
        }),

        ("genesis", 3, 23) => Some(Verse {
            content: "Therefore the LORD God sent him foorth from the garden of Eden, to till the ground, from whence he was taken.",
        }),

        ("genesis", 3, 24) => Some(Verse {
            content: "So he droue out the man: and he placed at the East of the garden of Eden, Cherubims, and a flaming sword, which turned euery way, to keepe the way of the tree of life.",
        }),

        ("genesis", 4, 1) => Some(Verse {
            content: "And Adam knew Eue his wife, and shee conceiued, and bare Cain, and said, I haue gotten a man from the LORD.",
        }),

        ("genesis", 4, 2) => Some(Verse {
            content: "And she againe bare his brother Abel, and Abel was a keeper of sheep, but Cain was a tiller of the ground.",
        }),

        ("genesis", 4, 3) => Some(Verse {
            content: "And in processe of time it came to passe, that Cain brought of the fruite of the ground, an offering vnto the LORD.",
        }),

        ("genesis", 4, 4) => Some(Verse {
            content: "And Abel, he also brought of the firstlings of his flocke, and of the fat thereof: and the LORD had respect vnto Abel, and to his offering.",
        }),

        ("genesis", 4, 5) => Some(Verse {
            content: "But vnto Cain, and to his offring he had not respect: and Cain was very wroth, and his countenance fell.",
        }),

        ("genesis", 4, 6) => Some(Verse {
            content: "And the LORD said vnto Cain, Why art thou wroth? And why is thy countenance fallen?",
        }),

        ("genesis", 4, 7) => Some(Verse {
            content: "If thou doe well, shalt thou not be accepted? and if thou doest not well, sinne lieth at the doore: And vnto thee shall be his desire, and thou shalt rule ouer him.",
        }),

        ("genesis", 4, 8) => Some(Verse {
            content: "And Cain talked with Abel his brother: and it came to passe when they were in the field, that Cain rose vp against Abel his brother, and slew him.",
        }),

        ("genesis", 4, 9) => Some(Verse {
            content: "And the LORD said vnto Cain, Where is Abel thy brother? And hee said, I know not: Am I my brothers keeper?",
        }),

        ("genesis", 4, 10) => Some(Verse {
            content: "And he said, What hast thou done? the voyce of thy brothers blood cryeth vnto me, from the ground.",
        }),

        ("genesis", 4, 11) => Some(Verse {
            content: "And now art thou cursed from the earth, which hath opened her mouth to receiue thy brothers blood from thy hand.",
        }),

        ("genesis", 4, 12) => Some(Verse {
            content: "When thou tillest the ground, it shall not henceforth yeeld vnto thee her strength: A fugitiue and a vagabond shalt thou be in the earth.",
        }),

        ("genesis", 4, 13) => Some(Verse {
            content: "And Cain said vnto the LORD, My punishment is greater, then I can beare.",
        }),

        ("genesis", 4, 14) => Some(Verse {
            content: "Behold, thou hast driuen me out this day from the face of the earth, and from thy face shall I be hid, and I shall be a fugitiue, and a vagabond in the earth: and it shall come to passe, that euery one that findeth me, shall slay me.",
        }),

        ("genesis", 4, 15) => Some(Verse {
            content: "And the LORD said vnto him, Therefore whosoeuer slayeth Cain, vengeance shalbe taken on him seuen fold. And the LORD set a marke vpon Cain, lest any finding him, should kill him.",
        }),

        ("genesis", 4, 16) => Some(Verse {
            content: "And Cain went out from the presence of the LORD, and dwelt in the land of Nod, on the East of Eden.",
        }),

        ("genesis", 4, 17) => Some(Verse {
            content: "And Cain knew his wife, and she conceiued and bare Enoch, and hee builded a City, and called the name of the City, after the name of his sonne, Enoch.",
        }),

        ("genesis", 4, 18) => Some(Verse {
            content: "And vnto Enoch was borne Irad: and Irad begate Mehuiael, and Mehuiael begate Methusael, and Methusael begate Lamech.",
        }),

        ("genesis", 4, 19) => Some(Verse {
            content: "And Lamech tooke vnto him two wiues: the name of the one was Adah, and the name of the other Zillah.",
        }),

        ("genesis", 4, 20) => Some(Verse {
            content: "And Adah bare Iabal: he was the father of such as dwell in tents, and of such as haue cattell.",
        }),

        ("genesis", 4, 21) => Some(Verse {
            content: "And his brothers name was Iubal: hee was the father of all such as handle the harpe and organ.",
        }),

        ("genesis", 4, 22) => Some(Verse {
            content: "And Zillah, she also bare Tubal-Cain, an instructer of euery artificer in brasse and iron: and the sister of Tubal-Cain was Naamah.",
        }),

        ("genesis", 4, 23) => Some(Verse {
            content: "And Lamech sayd vnto his wiues, Adah and Zillah, Heare my voyce, yee wiues of Lamech, hearken vnto my speech: for I haue slaine a man to my wounding, and a yong man to my hurt.",
        }),

        ("genesis", 4, 24) => Some(Verse {
            content: "If Cain shall bee auenged seuen fold, truely Lamech seuenty and seuen folde.",
        }),

        ("genesis", 4, 25) => Some(Verse {
            content: "And Adam knew his wife againe, and she bare a sonne, & called his name Seth: For God, said she, hath appointed mee another seed in stead of Abel, whom Cain slew.",
        }),

        ("genesis", 4, 26) => Some(Verse {
            content: "And to Seth, to him also there was borne a sonne, and he called his name Enos: then began men to call vpon the Name of the LORD.",
        }),

        ("genesis", 5, 1) => Some(Verse {
            content: "This is the booke of the generations of Adam: In the day that God created man, in the likenes of God made he him.",
        }),

        ("genesis", 5, 2) => Some(Verse {
            content: "Male and female created hee them, and blessed them, and called their name Adam, in the day when they were created.",
        }),

        ("genesis", 5, 3) => Some(Verse {
            content: "And Adam liued an hundred and thirtie yeeres, and begate a sonne in his owne likenesse, after his image; and called his name Seth.",
        }),

        ("genesis", 5, 4) => Some(Verse {
            content: "And the dayes of Adam, after he had begotten Seth, were eight hundred yeeres: and he begate sonnes and daughters.",
        }),

        ("genesis", 5, 5) => Some(Verse {
            content: "And all the dayes that Adam liued, were nine hundred and thirtie yeeres: and he died.",
        }),

        ("genesis", 5, 6) => Some(Verse {
            content: "And Seth liued an hundred and fiue yeeres: and begate Enos.",
        }),

        ("genesis", 5, 7) => Some(Verse {
            content: "And Seth liued, after he begate Enos, eight hundred and seuen yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 5, 8) => Some(Verse {
            content: "And all the dayes of Seth, were nine hundred and twelue yeeres, and he died.",
        }),

        ("genesis", 5, 9) => Some(Verse {
            content: "And Enos liued ninetie yeeres, and begate Cainan.",
        }),

        ("genesis", 5, 10) => Some(Verse {
            content: "And Enos liued after hee begate Cainan, eight hundred and fifteene yeeres, and begate sonnes & daughters.",
        }),

        ("genesis", 5, 11) => Some(Verse {
            content: "And all the dayes of Enos were nine hundred & fiue yeres; and he died.",
        }),

        ("genesis", 5, 12) => Some(Verse {
            content: "And Cainan liued seuentie yeeres, and begate Mahalaleel.",
        }),

        ("genesis", 5, 13) => Some(Verse {
            content: "And Cainan liued after he begate Mahalaleel, eight hundred and fourtie yeeres, & begate sonnes and daughters.",
        }),

        ("genesis", 5, 14) => Some(Verse {
            content: "And al the dayes of Cainan were nine hundred & ten yeres; and he died.",
        }),

        ("genesis", 5, 15) => Some(Verse {
            content: "And Mahalaleel liued sixtie and fiue yeeres, and begat Iared.",
        }),

        ("genesis", 5, 16) => Some(Verse {
            content: "And Mahalaleel liued after he begate Iared, eight hundred and thirtie yeeres, and begate sonnes & daughters.",
        }),

        ("genesis", 5, 17) => Some(Verse {
            content: "And all the dayes of Mahalaleel, were eight hundred ninetie and fiue yeeres, and he died.",
        }),

        ("genesis", 5, 18) => Some(Verse {
            content: "And Iared liued an hundred sixtie and two yeeres, & he begat Enoch.",
        }),

        ("genesis", 5, 19) => Some(Verse {
            content: "And Iared liued after he begate Enoch, eight hundred yeres, and begate sonnes and daughters.",
        }),

        ("genesis", 5, 20) => Some(Verse {
            content: "And all the dayes of Iared were nine hundred sixtie and two yeeres, and he died.",
        }),

        ("genesis", 5, 21) => Some(Verse {
            content: "And Enoch liued sixtie and fiue yeeres, and begate Methuselah.",
        }),

        ("genesis", 5, 22) => Some(Verse {
            content: "And Enoch walked with God, after he begate Methuselah, three hundred yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 5, 23) => Some(Verse {
            content: "And all the dayes of Enoch, were three hundred sixtie and fiue yeeres.",
        }),

        ("genesis", 5, 24) => Some(Verse {
            content: "And Enoch walked with God: and he was not; for God tooke him.",
        }),

        ("genesis", 5, 25) => Some(Verse {
            content: "And Methuselah liued an hundred eightie and seuen yeeres, and begat Lamech.",
        }),

        ("genesis", 5, 26) => Some(Verse {
            content: "And Methuselah liued, after hee begate Lamech, seuen hundred, eightie and two yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 5, 27) => Some(Verse {
            content: "And all the dayes of Methuselah were nine hundred, sixtie and nine yeeres, and he died.",
        }),

        ("genesis", 5, 28) => Some(Verse {
            content: "And Lamech liued an hundred eightie and two yeeres: and begate a sonne.",
        }),

        ("genesis", 5, 29) => Some(Verse {
            content: "And he called his name Noah, saying; This same shall comfort vs, concerning our woorke and toyle of our hands, because of the ground, which the LORD hath cursed.",
        }),

        ("genesis", 5, 30) => Some(Verse {
            content: "And Lamech liued, after hee begate Noah, fiue hundred ninetie and fiue yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 5, 31) => Some(Verse {
            content: "And all the dayes of Lamech were seuen hundred seuentie and seuen yeeres, and he died.",
        }),

        ("genesis", 5, 32) => Some(Verse {
            content: "And Noah was fiue hundred yeeres olde: and Noah begate Sem, Ham, and Iapheth.",
        }),

        ("genesis", 6, 1) => Some(Verse {
            content: "And it came to passe, when men began to multiply on the face of the earth, and daughters were borne vnto them:",
        }),

        ("genesis", 6, 2) => Some(Verse {
            content: "That the sonnes of God saw the daughters of men, that they were faire, and they took them wiues, of all which they chose.",
        }),

        ("genesis", 6, 3) => Some(Verse {
            content: "And the LORD said, My Spirit shall not alwayes striue with man; for that hee also is flesh: yet his dayes shalbe an hundred and twenty yeeres.",
        }),

        ("genesis", 6, 4) => Some(Verse {
            content: "There were Giants in the earth in those daies: and also after that, when the sonnes of God came in vnto the daughters of men, & they bare children to them; the same became mightie men, which were of old, men of renowme.",
        }),

        ("genesis", 6, 5) => Some(Verse {
            content: "And God saw, that the wickednes of man was great in the earth, and that euery imagination of the thoughts of his heart was onely euill continually.",
        }),

        ("genesis", 6, 6) => Some(Verse {
            content: "And it repented the LORD that he had made man on the earth, and it grieued him at his heart.",
        }),

        ("genesis", 6, 7) => Some(Verse {
            content: "And the LORD said, I will destroy man, whom I haue created, from the face of the earth: both man and beast, and the creeping thing, and the foules of the aire: for it repenteth me that I haue made them.",
        }),

        ("genesis", 6, 8) => Some(Verse {
            content: "But Noah found grace in the eyes of the LORD.",
        }),

        ("genesis", 6, 9) => Some(Verse {
            content: "These are the generations of Noah: Noah was a iust man, and perfect in his generations, and Noah walked with God.",
        }),

        ("genesis", 6, 10) => Some(Verse {
            content: "And Noah begate three sonnes: Sem, Ham, and Iapheth.",
        }),

        ("genesis", 6, 11) => Some(Verse {
            content: "The earth also was corrupt before God; and the earth was filled with violence.",
        }),

        ("genesis", 6, 12) => Some(Verse {
            content: "And God looked vpon the earth, and behold, it was corrupt: for all flesh had corrupted his way vpon the earth.",
        }),

        ("genesis", 6, 13) => Some(Verse {
            content: "And God said vnto Noah, The end of all flesh is come before mee; for the earth is filled with violence through them; and behold, I will destroy them with the earth.",
        }),

        ("genesis", 6, 14) => Some(Verse {
            content: "Make thee an Arke of Gopher-wood: roomes shalt thou make in the arke, and shalt pitch it within and without with pitch.",
        }),

        ("genesis", 6, 15) => Some(Verse {
            content: "And this is the fashion, which thou shalt make it of: the length of the arke shalbe three hundred cubits, the breadth of it fifty cubits, and the height of it thirtie cubits.",
        }),

        ("genesis", 6, 16) => Some(Verse {
            content: "A window shalt thou make to the arke, and in a cubite shalt thou finish it aboue; and the doore of the arke shalt thou set in the side thereof: With lower, second, and third stories shalt thou make it.",
        }),

        ("genesis", 6, 17) => Some(Verse {
            content: "And behold, I, euen I doe bring a flood of waters vpon the earth, to destroy all flesh, wherein is the breath of life from vnder heauen, and euery thing that is in the earth shall die.",
        }),

        ("genesis", 6, 18) => Some(Verse {
            content: "But with thee wil I establish my Couenant: and thou shalt come into the Arke, thou, and thy sonnes, and thy wife, and thy sonnes wiues with thee.",
        }),

        ("genesis", 6, 19) => Some(Verse {
            content: "And of euery liuing thing of all flesh, two of euery sort shalt thou bring into the Arke, to keepe them aliue with thee: they shall be male and female.",
        }),

        ("genesis", 6, 20) => Some(Verse {
            content: "Of fowles after their kinde, and of cattel after their kinde: of euery creeping thing of the earth after his kinde, two of euery sort shall come vnto thee, to keepe them aliue.",
        }),

        ("genesis", 6, 21) => Some(Verse {
            content: "And take thou vnto thee of all food that is eaten, and thou shalt gather it to thee; and it shall be for food, for thee, and for them.",
        }),

        ("genesis", 6, 22) => Some(Verse {
            content: "Thus did Noah; according to all that God commanded him, so did he.",
        }),

        ("genesis", 7, 1) => Some(Verse {
            content: "And the LORD saide vnto Noah, Come thou and all thy house into the Arke: for thee haue I seene righteous before me, in this generation.",
        }),

        ("genesis", 7, 2) => Some(Verse {
            content: "Of euery cleane beast thou shalt take to thee by seuens, the male and his female: and of beastes that are not cleane, by two, the male and his female.",
        }),

        ("genesis", 7, 3) => Some(Verse {
            content: "Of fowles also of the aire, by seuens, the male & the female; to keepe seed aliue vpon the face of all the earth.",
        }),

        ("genesis", 7, 4) => Some(Verse {
            content: "For yet seuen dayes, and I will cause it to raine vpon the earth, fortie dayes, and forty nights: and euery liuing substance that I haue made, will I destroy, frō off the face of the earth.",
        }),

        ("genesis", 7, 5) => Some(Verse {
            content: "And Noah did according vnto all that the LORD commanded him.",
        }),

        ("genesis", 7, 6) => Some(Verse {
            content: "And Noah was sixe hundred yeeres old, when the flood of waters was vpon the earth.",
        }),

        ("genesis", 7, 7) => Some(Verse {
            content: "And Noah went in, and his sonnes, and his wife, and his sonnes wiues with him, into the Arke, because of the waters of the Flood.",
        }),

        ("genesis", 7, 8) => Some(Verse {
            content: "Of cleane beasts, & of beasts that are not cleane, & of fowles, and of euery thing that creepeth vpon the earth,",
        }),

        ("genesis", 7, 9) => Some(Verse {
            content: "There went in two and two vnto Noah into the Arke, the male & the female, as God had commanded Noah.",
        }),

        ("genesis", 7, 10) => Some(Verse {
            content: "And it came to passe after seuen dayes, that the waters of the Flood were vpon the earth.",
        }),

        ("genesis", 7, 11) => Some(Verse {
            content: "In the sixe hundredth yeere of Noahs life, in the second moneth, the seuenteenth day of the moneth, the same day, were al the fountaines of the great deepe broken vp, and the windowes of heauen were opened.",
        }),

        ("genesis", 7, 12) => Some(Verse {
            content: "In the selfe same day entred Noah, and Sem, and Ham, and Iapheth, the sonnes of Noah, and Noahs wife, and the three wiues of his sonnes with them, into the Arke,",
        }),

        ("genesis", 7, 13) => Some(Verse {
            content: "In the selfe same day entred Noah, and Sem, and Ham, and Iapheth, the sonnes of Noah, and Noahs wife, and the three wiues of his sonnes with them, into the Arke,",
        }),

        ("genesis", 7, 14) => Some(Verse {
            content: "They, and euery beast after his kinde, & all the cattell after their kinde: and euery creeping thing that creepeth vpon the earth after his kinde, and euery foule after his kinde, euery birde of euery sort.",
        }),

        ("genesis", 7, 15) => Some(Verse {
            content: "And they went in vnto Noah into the Arke, two and two of all flesh, wherein is the breath of life.",
        }),

        ("genesis", 7, 16) => Some(Verse {
            content: "And they that went in, went in male and female of all flesh, as God had commaunded him: and the LORD shut him in.",
        }),

        ("genesis", 7, 17) => Some(Verse {
            content: "And the Flood was fortie dayes vpon the earth, and the waters increased, and bare vp the Arke, and it was lift vp aboue the earth.",
        }),

        ("genesis", 7, 18) => Some(Verse {
            content: "And the waters preuailed, and were encreased greatly vpon the earth: and the Arke went vpon the face of the waters.",
        }),

        ("genesis", 7, 19) => Some(Verse {
            content: "And the waters preuailed exceedingly vpon the earth, and all the high hils, that were vnder the whole heauen, were couered.",
        }),

        ("genesis", 7, 20) => Some(Verse {
            content: "Fifteene cubits vpward, did the waters preuaile; and the mountaines were couered.",
        }),

        ("genesis", 7, 21) => Some(Verse {
            content: "And all flesh died, that mooued vpon the earth, both of fowle, & of cattell, and of beast, and of euery creeping thing that creepeth vpon the earth, and euery man.",
        }),

        ("genesis", 7, 22) => Some(Verse {
            content: "All in whose nosethrils was the breath of life, of all that was in the dry land, died.",
        }),

        ("genesis", 7, 23) => Some(Verse {
            content: "And euery liuing substance was destroyed, which was vpon the face of the ground, both man and cattell, and the creeping things, and the foule of the heauen; and they were destroyed from the earth: and Noah onely remained aliue, and they that were with him in the Arke.",
        }),

        ("genesis", 7, 24) => Some(Verse {
            content: "And the waters preuailed vpon the earth, an hundred and fifty dayes.",
        }),

        ("genesis", 8, 1) => Some(Verse {
            content: "And God remembred Noah, and euery liuing thing, and all the cattell that was with him in the Arke: and God made a winde to passe ouer the earth, and the waters asswaged.",
        }),

        ("genesis", 8, 2) => Some(Verse {
            content: "The fountaines also of the deepe, and the windowes of heauen were stopped, and the raine from heauen was restrained.",
        }),

        ("genesis", 8, 3) => Some(Verse {
            content: "And the waters returned from off the earth, continually: and after the end of the hundred and fiftie dayes, the waters were abated.",
        }),

        ("genesis", 8, 4) => Some(Verse {
            content: "And the Arke rested in the seuenth moneth, on the seuenteenth day of the moneth, vpon the mountaines of Ararat.",
        }),

        ("genesis", 8, 5) => Some(Verse {
            content: "And the waters decreased continually vntill the tenth moneth: in the tenth moneth, on the first day of the moneth, were the tops of the mountaines seene.",
        }),

        ("genesis", 8, 6) => Some(Verse {
            content: "And it came to passe at the end of forty dayes, that Noah opened the window of the Arke which he had made.",
        }),

        ("genesis", 8, 7) => Some(Verse {
            content: "And he sent forth a Rauen, which went foorth to and fro, vntill the waters were dried vp from off the earth.",
        }),

        ("genesis", 8, 8) => Some(Verse {
            content: "Also hee sent foorth a doue from him, to see if the waters were abated from off the face of the ground.",
        }),

        ("genesis", 8, 9) => Some(Verse {
            content: "But the doue found no rest for the sole of her foote, and she returned vnto him into the Arke: for the waters were on the face of the whole earth. Then he put foorth his hand, and tooke her, and pulled her in vnto him, into the Arke.",
        }),

        ("genesis", 8, 10) => Some(Verse {
            content: "And hee stayed yet other seuen dayes; and againe hee sent foorth the doue out of the Arke.",
        }),

        ("genesis", 8, 11) => Some(Verse {
            content: "And the doue came in to him in the euening, and loe, in her mouth was an Oliue leafe pluckt off: So Noah knew that the waters were abated from off the earth.",
        }),

        ("genesis", 8, 12) => Some(Verse {
            content: "And hee stayed yet other seuen dayes, and sent forth the doue, which returned not againe vnto him any more.",
        }),

        ("genesis", 8, 13) => Some(Verse {
            content: "And it came to passe in the sixe hundredth and one yeere, in the first moneth, the first day of the moneth, the waters were dryed vp from off the earth: and Noah remooued the couering of the Arke, and looked, and behold, the face of the ground was drie.",
        }),

        ("genesis", 8, 14) => Some(Verse {
            content: "And in the second moneth, on the seuen and twentieth day of the moneth, was the earth dried.",
        }),

        ("genesis", 8, 15) => Some(Verse {
            content: "And God spake vnto Noah, saying,",
        }),

        ("genesis", 8, 16) => Some(Verse {
            content: "Goe foorth of the Arke, thou, and thy wife, and thy sonnes, and thy sonnes wiues with thee:",
        }),

        ("genesis", 8, 17) => Some(Verse {
            content: "Bring foorth with thee euery liuing thing that is with thee, of all flesh, both of fowle, and of cattell, and of euery creeping thing that creepeth vpon the earth, that they may breed abundantly in the earth, and be fruitfull, and multiply vpon the earth.",
        }),

        ("genesis", 8, 18) => Some(Verse {
            content: "And Noah went foorth, and his sonnes, and his wife, and his sonnes wiues with him:",
        }),

        ("genesis", 8, 19) => Some(Verse {
            content: "Euery beast, euery creeping thing, and euery fowle, and whatsoeuer creepeth vpon the earth, after their kinds, went foorth out of the Arke.",
        }),

        ("genesis", 8, 20) => Some(Verse {
            content: "And Noah builded an Altar vnto the LORD, and tooke of euery cleane beast, and of euery cleane fowle, and offred burnt offrings on the Altar.",
        }),

        ("genesis", 8, 21) => Some(Verse {
            content: "And the LORD smelled a sweete sauour, and the LORD said in his heart, I will not againe curse the ground any more for mans sake; for the imagination of mans heart is euil from his youth: neither will I againe smite any more euery thing liuing, as I haue done.",
        }),

        ("genesis", 8, 22) => Some(Verse {
            content: "While the earth remaineth, seed-time and haruest, and cold, and heat, and Summer, and Winter, and day and night, shall not cease.",
        }),

        ("genesis", 9, 1) => Some(Verse {
            content: "And God blessed Noah, and his sonnes, and said vnto them, Bee fruitfull and multiply, and replenish the earth.",
        }),

        ("genesis", 9, 2) => Some(Verse {
            content: "And the feare of you, & the dread of you shall be vpon euery beast of the earth, and vpon euery fowle of the aire, vpon all that mooueth vpon the earth, and vpon all the fishes of the sea; into your hand are they deliuered.",
        }),

        ("genesis", 9, 3) => Some(Verse {
            content: "Euery mouing thing that liueth, shalbe meat for you; euen as the greene herbe haue I giuen you all things.",
        }),

        ("genesis", 9, 4) => Some(Verse {
            content: "But flesh with the life thereof, which is the blood thereof, shall you not eate.",
        }),

        ("genesis", 9, 5) => Some(Verse {
            content: "And surely your blood of your liues will I require: at the hand of euery beast will I require it, & at the hand of man, at the hand of euery mans brother will I require the life of man.",
        }),

        ("genesis", 9, 6) => Some(Verse {
            content: "Who so sheddeth mans blood, by man shall his blood be shed: for in the image of God made he man.",
        }),

        ("genesis", 9, 7) => Some(Verse {
            content: "And you, be ye fruitfull, and multiply, bring foorth aboundantly in the earth, and multiply therein.",
        }),

        ("genesis", 9, 8) => Some(Verse {
            content: "And God spake vnto Noah, and to his sonnes with him, saying;",
        }),

        ("genesis", 9, 9) => Some(Verse {
            content: "And I, behold, I establish my couenant with you, and with your seede after you:",
        }),

        ("genesis", 9, 10) => Some(Verse {
            content: "And with euery liuing creature that is with you, of the fowle, of the cattell, and of euery beast of the earth with you, from all that goe out of the Arke, to euery beast of the earth.",
        }),

        ("genesis", 9, 11) => Some(Verse {
            content: "And I wil establish my couenant with you, neither shal all flesh be cut off any more, by the waters of a flood, neither shall there any more be a flood to destroy the earth.",
        }),

        ("genesis", 9, 12) => Some(Verse {
            content: "And God said, This is the token of the Couenant which I make betweene mee and you, and euery liuing creature that is with you, for perpetuall generations.",
        }),

        ("genesis", 9, 13) => Some(Verse {
            content: "I doe set my bow in the cloud, and it shall be for a token of a couenant, betweene me and the earth.",
        }),

        ("genesis", 9, 14) => Some(Verse {
            content: "And I will remember my couenant, which is betweene mee and you, and euery liuing creature of all flesh: and the waters shall no more become a flood to destroy all flesh.",
        }),

        ("genesis", 9, 15) => Some(Verse {
            content: "And I will remember my couenant, which is betweene mee and you, and euery liuing creature of all flesh: and the waters shall no more become a flood to destroy all flesh.",
        }),

        ("genesis", 9, 16) => Some(Verse {
            content: "And the bow shalbe in the cloud; and I will looke vpon it, that I may remember the euerlasting couenant betweene God and euery liuing creature, of all flesh that is vpon the earth.",
        }),

        ("genesis", 9, 17) => Some(Verse {
            content: "And God said vnto Noah, This is the token of the couenant, which I haue established betweene mee and all flesh, that is vpon the earth.",
        }),

        ("genesis", 9, 18) => Some(Verse {
            content: "And the sonnes of Noah that went forth of the Arke, were Shem, and Ham, and Iaphet: and Ham is the father of Canaan.",
        }),

        ("genesis", 9, 19) => Some(Verse {
            content: "These are the three sonnes of Noah: and of them was the whole earth ouerspread.",
        }),

        ("genesis", 9, 20) => Some(Verse {
            content: "And Noah began to bee an husbandman, and he planted a vineyard.",
        }),

        ("genesis", 9, 21) => Some(Verse {
            content: "And he dranke of the wine, and was drunken, and hee was vncouered within his tent.",
        }),

        ("genesis", 9, 22) => Some(Verse {
            content: "And Ham, the father of Canaan, saw the nakednesse of his father, and told his two brethren without.",
        }),

        ("genesis", 9, 23) => Some(Verse {
            content: "And Shem and Iaphet tooke a garment, and layed it vpon both their shoulders, and went backward, and couered the nakednesse of their father, and their faces were backward, and they saw not their fathers nakednesse.",
        }),

        ("genesis", 9, 24) => Some(Verse {
            content: "And Noah awoke from his wine, and knew what his yonger sonne had done vnto him.",
        }),

        ("genesis", 9, 25) => Some(Verse {
            content: "And he said, Cursed bee Canaan: a seruant of seruants shall hee be vnto his brethren.",
        }),

        ("genesis", 9, 26) => Some(Verse {
            content: "And hee saide, Blessed bee the LORD God of Shem, and Canaan shalbe his seruant.",
        }),

        ("genesis", 9, 27) => Some(Verse {
            content: "God shall enlarge Iaphet, and he shal dwel in the tents of Shem, and Canaan shalbe his seruant.",
        }),

        ("genesis", 9, 28) => Some(Verse {
            content: "And Noah liued after the flood, three hundred and fifty yeeres.",
        }),

        ("genesis", 9, 29) => Some(Verse {
            content: "And all the dayes of Noah were nine hundred & fifty yeeres, and he died.",
        }),

        ("genesis", 10, 1) => Some(Verse {
            content: "Now these are the generations of the sonnes of Noah; Shem, Ham, and Iaphet: and vnto them were sonnes borne after the Flood.",
        }),

        ("genesis", 10, 2) => Some(Verse {
            content: "The sonnes of Iaphet: Gomer, and Magog, and Madai, and Iauan, & Tubal, and Meshech, & Tiras.",
        }),

        ("genesis", 10, 3) => Some(Verse {
            content: "And the sonnes of Gomer: Ashkenaz, and Riphath, and Togarmah.",
        }),

        ("genesis", 10, 4) => Some(Verse {
            content: "And the sons of Iauan: Elishah, and Tarshish, Kittim, and Dodanim.",
        }),

        ("genesis", 10, 5) => Some(Verse {
            content: "By these were the Iles of the Gentiles diuided in their lands, euery one after his tongue: after their families, in their nations.",
        }),

        ("genesis", 10, 6) => Some(Verse {
            content: "And the sonnes of Ham: Cush, and Mizraim, and Phut, and Canaan.",
        }),

        ("genesis", 10, 7) => Some(Verse {
            content: "And the sonnes of Cush, Seba, and Hauilah, and Sabtah, and Raamah, and Sabtecha: and the sonnes of Raamah: Sheba, and Dedan.",
        }),

        ("genesis", 10, 8) => Some(Verse {
            content: "And Cush begat Nimrod: he began to be a mighty one in the earth.",
        }),

        ("genesis", 10, 9) => Some(Verse {
            content: "He was a mighty hunter before the LORD: wherefore it is saide, Euen as Nimrod the mightie hunter before the LORD.",
        }),

        ("genesis", 10, 10) => Some(Verse {
            content: "And the beginning of his kingdome was Babel, and Erech, and Accad, and Calneh, in the land of Shinar.",
        }),

        ("genesis", 10, 11) => Some(Verse {
            content: "Out of that land went forth Asshur, and builded Nineueh, and the citie Rehoboth, and Calah,",
        }),

        ("genesis", 10, 12) => Some(Verse {
            content: "And Resen betweene Nineueh and Calah: the same is a great citie.",
        }),

        ("genesis", 10, 13) => Some(Verse {
            content: "And Mizraim begat Ludim, and Anamim, and Lehabim, and Naphtuhim,",
        }),

        ("genesis", 10, 14) => Some(Verse {
            content: "And Pathrusim, and Casluhim (out of whome came Philistiim) and Caphtorim.",
        }),

        ("genesis", 10, 15) => Some(Verse {
            content: "And Canaan begate Sidon his first borne, and Heth,",
        }),

        ("genesis", 10, 16) => Some(Verse {
            content: "And the Iebusite, and the Emorite, and the Girgasite,",
        }),

        ("genesis", 10, 17) => Some(Verse {
            content: "And the Hiuite, and the Arkite, and the Sinite,",
        }),

        ("genesis", 10, 18) => Some(Verse {
            content: "And the Aruadite, and the Zemarite, and the Hamathite: and afterward were the families of the Canaanites spread abroad.",
        }),

        ("genesis", 10, 19) => Some(Verse {
            content: "And the border of the Canaanites, was from Sidon, as thou commest to Gerar, vnto Gaza, as thou goest vnto Sodoma and Gomorah, and Admah, & Zeboim, euen vnto Lasha.",
        }),

        ("genesis", 10, 20) => Some(Verse {
            content: "These are the sonnes of Ham, after their families, after their tongues, in their countries, and in their nations.",
        }),

        ("genesis", 10, 21) => Some(Verse {
            content: "Unto Shem also the father of all the children of Eber, the brother of Iaphet the elder, euen to him were children borne.",
        }),

        ("genesis", 10, 22) => Some(Verse {
            content: "The children of Shem: Elam, and Asshur, and Arphaxad, and Lud, and Aram.",
        }),

        ("genesis", 10, 23) => Some(Verse {
            content: "And the children of Aram: Uz, and Hul, and Gether, and Mash.",
        }),

        ("genesis", 10, 24) => Some(Verse {
            content: "And Arphaxad begate Salah, and Salah begate Eber.",
        }),

        ("genesis", 10, 25) => Some(Verse {
            content: "And vnto Eber were borne two sonnes: the name of one was Peleg, for in his dayes was the earth diuided, and his brothers name was Ioktan.",
        }),

        ("genesis", 10, 26) => Some(Verse {
            content: "And Ioktan begate Almodad, and Sheleph, and Hazarmaueth, and Ierah,",
        }),

        ("genesis", 10, 27) => Some(Verse {
            content: "And Hadoram, and Uzal, and Diklah,",
        }),

        ("genesis", 10, 28) => Some(Verse {
            content: "And Obal, and Abimael, and Sheba,",
        }),

        ("genesis", 10, 29) => Some(Verse {
            content: "And Ophir, and Hauilah, & Iobab: all these were the sonnes of Ioktan.",
        }),

        ("genesis", 10, 30) => Some(Verse {
            content: "And their dwelling was from Mesha, as thou goest vnto Sephar, a mount of the East.",
        }),

        ("genesis", 10, 31) => Some(Verse {
            content: "These are the sonnes of Shem, after their families, after their tongues, in their lands after their nations.",
        }),

        ("genesis", 10, 32) => Some(Verse {
            content: "These are the families of the sonnes of Noah after their generations, in their nations: and by these were the nations diuided in the earth after the Flood.",
        }),

        ("genesis", 11, 1) => Some(Verse {
            content: "And the whole earth was of one language, and of one speach.",
        }),

        ("genesis", 11, 2) => Some(Verse {
            content: "And it came to passe as they iourneyed from the East, that they found a plaine in the land of Shinar, and they dwelt there.",
        }),

        ("genesis", 11, 3) => Some(Verse {
            content: "And they sayd one to another; Goe to, let vs make bricke, and burne them thorowly. And they had bricke for stone, and slime had they for morter.",
        }),

        ("genesis", 11, 4) => Some(Verse {
            content: "And they said; Goe to, let vs build vs a city and a tower, whose top may reach vnto heauen, and let vs make vs a name, lest we be scattered abroad vpon the face of the whole earth.",
        }),

        ("genesis", 11, 5) => Some(Verse {
            content: "And the LORD came downe to see the city and the tower, which the children of men builded.",
        }),

        ("genesis", 11, 6) => Some(Verse {
            content: "And the LORD said; Behold, the people is one, and they haue all one language: and this they begin to doe: and now nothing will be restrained from them, which they haue imagined to doe.",
        }),

        ("genesis", 11, 7) => Some(Verse {
            content: "Goe to, let vs go downe, and there cōfound their language, that they may not vnderstand one anothers speech.",
        }),

        ("genesis", 11, 8) => Some(Verse {
            content: "So the LORD scattered them abroad from thence, vpon the face of all the earth: and they left off to build the Citie.",
        }),

        ("genesis", 11, 9) => Some(Verse {
            content: "Therefore is the name of it called Babel, because the LORD did there confound the language of all the earth: and from thence did the LORD scatter them abroad vpon the face of all the earth.",
        }),

        ("genesis", 11, 10) => Some(Verse {
            content: "These are the generations of Shem. Shem was an hundred yeres old, and begate Arphaxad two yeeres after the Flood.",
        }),

        ("genesis", 11, 11) => Some(Verse {
            content: "And Shem liued, after he begate Arphaxad, fiue hundred yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 12) => Some(Verse {
            content: "And Arphaxad liued fiue and thirtie yeeres, and begate Salah.",
        }),

        ("genesis", 11, 13) => Some(Verse {
            content: "And Arphaxad liued, after he begate Salah, foure hundred and three yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 14) => Some(Verse {
            content: "And Salah liued thirtie yeeres, and begate Eber.",
        }),

        ("genesis", 11, 15) => Some(Verse {
            content: "And Salah liued, after hee begate Eber, foure hundred and three yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 16) => Some(Verse {
            content: "And Eber liued foure and thirty yeeres, and begate Peleg.",
        }),

        ("genesis", 11, 17) => Some(Verse {
            content: "And Eber liued, after hee begate Peleg, foure hundred and thirtie yeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 18) => Some(Verse {
            content: "And Peleg liued thirtie yeeres, and begate Reu.",
        }),

        ("genesis", 11, 19) => Some(Verse {
            content: "And Peleg liued, after hee begate Reu, two hundred and nine yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 20) => Some(Verse {
            content: "And Reu liued two and thirtie yeeres, and begate Serug.",
        }),

        ("genesis", 11, 21) => Some(Verse {
            content: "And Reu liued, after hee begate Serug, two hundreth and seuen yeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 22) => Some(Verse {
            content: "And Serug liued thirtie yeeres, and begate Nahor.",
        }),

        ("genesis", 11, 23) => Some(Verse {
            content: "And Serug liued, after he begate Nahor, two hundred yeeres, and begat sonnes and daughters.",
        }),

        ("genesis", 11, 24) => Some(Verse {
            content: "And Nahor liued nine and twentie yeeres, and begate Terah.",
        }),

        ("genesis", 11, 25) => Some(Verse {
            content: "And Nahor liued, after he begate Terah, an hundred & nineteene yeeres, and begate sonnes and daughters.",
        }),

        ("genesis", 11, 26) => Some(Verse {
            content: "And Terah liued seuenty yeeres, and begate Abram, Nahor, & Haran.",
        }),

        ("genesis", 11, 27) => Some(Verse {
            content: "Now these are the generations of Terah: Terah begate Abram, Nahor, and Haran: And Haran begate Lot.",
        }),

        ("genesis", 11, 28) => Some(Verse {
            content: "And Haran died, before his father Terah in the land of his natiuity, in Ur of the Chaldees.",
        }),

        ("genesis", 11, 29) => Some(Verse {
            content: "And Abram and Nahor tooke them wiues: the name of Abrams wife was Sarai, and the name of Nahors wife, Milcah, the daughter of Haran, the father of Milcah, and the father of Iscah.",
        }),

        ("genesis", 11, 30) => Some(Verse {
            content: "But Sarai was barren; she had no childe.",
        }),

        ("genesis", 11, 31) => Some(Verse {
            content: "And Terah tooke Abram his sonne, and Lot the sonne of Haran his sonnes sonne, and Sarai his daughter in lawe, his sonne Abrams wife, and they went foorth with them from Ur of the Chaldees, to goe into the land of Canaan: and they came vnto Haran, and dwelt there.",
        }),

        ("genesis", 11, 32) => Some(Verse {
            content: "And the dayes of Terah, were two hundred and fiue yeres: and Terah died in Haran.",
        }),

        ("genesis", 12, 1) => Some(Verse {
            content: "Now the LORD had said vnto Abram, Get thee out of thy countrey, and from thy kinred, and from thy fathers house, vnto a land that I will shew thee.",
        }),

        ("genesis", 12, 2) => Some(Verse {
            content: "And I will make of thee a great nation, and I wil blesse thee, and make thy name great; and thou shalt bee a blessing.",
        }),

        ("genesis", 12, 3) => Some(Verse {
            content: "And I will blesse them that blesse thee, and curse him, that curseth thee: and in thee shal all families of the earth be blessed.",
        }),

        ("genesis", 12, 4) => Some(Verse {
            content: "So Abram departed, as the LORD had spoken vnto him, and Lot went with him: And Abram was seuentie and fiue yeeres old when he departed out of Haran.",
        }),

        ("genesis", 12, 5) => Some(Verse {
            content: "And Abram tooke Sarai his wife, and Lot his brothers sonne, and all their substance that they had gathered, and the soules that they had gotten in Haran, and they went foorth to goe into the land of Canaan: and into the land of Canaan they came.",
        }),

        ("genesis", 12, 6) => Some(Verse {
            content: "And Abram passed through the land, vnto the place of Sichem, vnto the plaine of Moreh. And the Canaanite was then in the land.",
        }),

        ("genesis", 12, 7) => Some(Verse {
            content: "And the LORD appeared vnto Abram, and said, Unto thy seed wil I giue this land: and there builded hee an altar vnto the LORD, who appeared vnto him.",
        }),

        ("genesis", 12, 8) => Some(Verse {
            content: "And he remoued from thence vnto a mountaine, on the East of Beth-el, and pitched his tent hauing Beth-el on the West, and Hai on the East: and there hee builded an altar vnto the LORD, and called vpon the Name of the LORD.",
        }),

        ("genesis", 12, 9) => Some(Verse {
            content: "And Abram iourneyed, going on still toward the South.",
        }),

        ("genesis", 12, 10) => Some(Verse {
            content: "And there was a famine in the land, and Abram went downe into Egypt, to soiourne there: for the famine was grieuous in the land.",
        }),

        ("genesis", 12, 11) => Some(Verse {
            content: "And it came to passe when he was come neere to enter into Egypt, that he said vnto Sarai his wife, Behold now, I know that thou art a faire woman to looke vpon.",
        }),

        ("genesis", 12, 12) => Some(Verse {
            content: "Therefore it shall come to passe, when the Egyptians shall see thee, that they shall say, This is his wife: and they will kill me, but they will saue thee aliue.",
        }),

        ("genesis", 12, 13) => Some(Verse {
            content: "Say, I pray thee, thou art my sister, that it may be wel with me, for thy sake; and my soule shall liue, because of thee.",
        }),

        ("genesis", 12, 14) => Some(Verse {
            content: "And it came to passe, that when Abram was come into Egypt, the Egyptians beheld the woman, that shee was very faire.",
        }),

        ("genesis", 12, 15) => Some(Verse {
            content: "The Princes also of Pharaoh saw her, and commended her before Pharaoh: and the woman was taken into Pharaohs house.",
        }),

        ("genesis", 12, 16) => Some(Verse {
            content: "And he entreated Abram well for her sake: and he had sheepe, and oxen, and hee asses, and men seruants, and maid seruants, and shee asses, and camels.",
        }),

        ("genesis", 12, 17) => Some(Verse {
            content: "And the LORD plagued Pharaoh & his house with great plagues, because of Sarai Abrams wife.",
        }),

        ("genesis", 12, 18) => Some(Verse {
            content: "And Pharaoh called Abram, and said, What is this that thou hast done vnto me? Why diddest thou not tell me, that she was thy wife?",
        }),

        ("genesis", 12, 19) => Some(Verse {
            content: "Why saidest thou, Shee is my sister? so I might haue taken her to mee to wife: now therfore behold, thy wife, take her and goe thy way.",
        }),

        ("genesis", 12, 20) => Some(Verse {
            content: "And Pharaoh cōmanded his men concerning him: and they sent him away, and his wife, and all that he had.",
        }),

        ("genesis", 13, 1) => Some(Verse {
            content: "And Abram went vp out of Egypt, he and his wife, and all that he had, and Lot with him, into the South.",
        }),

        ("genesis", 13, 2) => Some(Verse {
            content: "And Abram was very rich in cattell, in siluer, and in gold.",
        }),

        ("genesis", 13, 3) => Some(Verse {
            content: "And hee went on his iourneyes from the South, euen to Beth-el, vnto the place where his tent had bene at the beginning, betweene Beth-el and Hai:",
        }),

        ("genesis", 13, 4) => Some(Verse {
            content: "Unto the place of the altar, which he had made there at the first: and there Abram called on the Name of the LORD.",
        }),

        ("genesis", 13, 5) => Some(Verse {
            content: "And Lot also which went with Abram, had flocks and heards, & tents.",
        }),

        ("genesis", 13, 6) => Some(Verse {
            content: "And the land was not able to beare them, that they might dwell together: for their substance was great, so that they could not dwell together.",
        }),

        ("genesis", 13, 7) => Some(Verse {
            content: "And there was a strife betweene the heardmen of Abrams cattell, and the heardmen of Lots cattell: And the Canaanite, and the Perizzite dwelled then in the land.",
        }),

        ("genesis", 13, 8) => Some(Verse {
            content: "And Abram said vnto Lot, Let there be no strife, I pray thee, betweene mee and thee, and betweene my heardmen and thy heardmen: for wee bee brethren.",
        }),

        ("genesis", 13, 9) => Some(Verse {
            content: "Is not the whole land before thee? Separate thy selfe, I pray thee, from mee: if thou wilt take the left hand, then I will goe to the right: or if thou depart to the right hand, then I will goe to the left.",
        }),

        ("genesis", 13, 10) => Some(Verse {
            content: "And Lot lifted vp his eyes, and beheld all the plaine of Iordane, that it was well watered euery where before the Lord destroyed Sodome and Gomorah, euen as the garden of the LORD, like the land of Egypt, as thou commest vnto Zoar.",
        }),

        ("genesis", 13, 11) => Some(Verse {
            content: "Then Lot chose him all the plaine of Iordane: and Lot iourneyed East; and they separated themselues the one from the other.",
        }),

        ("genesis", 13, 12) => Some(Verse {
            content: "Abram dwelled in the land of Canaan, and Lot dwelled in the cities of the plaine, and pitched his tent toward Sodome.",
        }),

        ("genesis", 13, 13) => Some(Verse {
            content: "But the men of Sodome were wicked, and sinners before the LORD exceedingly.",
        }),

        ("genesis", 13, 14) => Some(Verse {
            content: "And the LORD said vnto Abram, after that Lot was separated from him, Lift vp now thine eyes, and looke from the place where thou art, Northward, and Southward, and Eastward, and Westward.",
        }),

        ("genesis", 13, 15) => Some(Verse {
            content: "For all the land which thou seest, to thee will I give it, and to thy seede for euer.",
        }),

        ("genesis", 13, 16) => Some(Verse {
            content: "And I will make thy seede as the dust of the earth: so that if a man can number the dust of the earth, then shall thy seed also be numbred.",
        }),

        ("genesis", 13, 17) => Some(Verse {
            content: "Arise, walke through the land, in the length of it, and in the breadth of it: for I will giue it vnto thee.",
        }),

        ("genesis", 13, 18) => Some(Verse {
            content: "Then Abram remoued his tent, and came and dwelt in the plaine of Mamre, which is in Hebron, and built there an altar vnto the LORD.",
        }),

        ("genesis", 14, 1) => Some(Verse {
            content: "And it came to passe in the dayes of Amraphel King of Shinar, Arioch King of Ellasar, Chedorlaomer King of Elam, and Tidal King of nations:",
        }),

        ("genesis", 14, 2) => Some(Verse {
            content: "That these made warre with Bera King of Sodome, and with Birsha King of Gomorrah, Shinab King of Admah, and Shemeber King of Zeboiim, and the King of Bela, which is Zoar.",
        }),

        ("genesis", 14, 3) => Some(Verse {
            content: "All these were ioyned together in the vale of Siddim; which is the salt Sea.",
        }),

        ("genesis", 14, 4) => Some(Verse {
            content: "Twelue yeeres they serued Chedorlaomer, and in the thirteenth yeere they rebelled.",
        }),

        ("genesis", 14, 5) => Some(Verse {
            content: "And in the fourteenth yeere came Chedorlaomer, and the Kings that were with him, and smote the Rephaims, in Ashteroth Karnaim, & the Zuzims in Ham, and the Emims in Shaueh Kiriathaim;",
        }),

        ("genesis", 14, 6) => Some(Verse {
            content: "And the Horites in their mount Seir, vnto El-Paran, which is by the wildernesse.",
        }),

        ("genesis", 14, 7) => Some(Verse {
            content: "And they returned, and came to En-mishpat, which is Kadesh, & smote all the countrey of the Amalekites, and also the Amorites, that dwelt in Hazezon-tamar.",
        }),

        ("genesis", 14, 8) => Some(Verse {
            content: "And there went out the King of Sodome, and the King of Gomorrah, and the King of Admah, and the King of Zeboiim, and the King of Bela, (the same is Zoar) and they ioyned battell with them, in the vale of Siddim,",
        }),

        ("genesis", 14, 9) => Some(Verse {
            content: "With Chedorlaomer the King of Elam, and with Tidal King of nations, and Amraphel King of Shinar, and Arioch King of Ellasar; foure Kings with fiue.",
        }),

        ("genesis", 14, 10) => Some(Verse {
            content: "And the vale of Siddim was full of slime-pits: and the Kings of Sodome & Gomorrah fled, and fell there: and they that remained, fled to the mountaine.",
        }),

        ("genesis", 14, 11) => Some(Verse {
            content: "And they tooke all the goods of Sodome and Gomorrah, and all their victuals, and went their way.",
        }),

        ("genesis", 14, 12) => Some(Verse {
            content: "And they tooke Lot, Abrams brothers sonne, (who dwelt in Sodome) and his goods, and departed.",
        }),

        ("genesis", 14, 13) => Some(Verse {
            content: "And there came one that had escaped, and told Abram the Hebrew, for hee dwelt in the plaine of Mamre the Amorite, brother of Eshcol, and brother of Aner: and these were confederate with Abram.",
        }),

        ("genesis", 14, 14) => Some(Verse {
            content: "And when Abram heard that his brother was taken captiue, he armed his trained seruants borne in his owne house, three hundred and eighteene, and pursued them vnto Dan.",
        }),

        ("genesis", 14, 15) => Some(Verse {
            content: "And hee diuided himselfe against them, he and his seruants by night, and smote them, and pursued them vnto Hoba, which is on the left hand of Damascus:",
        }),

        ("genesis", 14, 16) => Some(Verse {
            content: "And hee brought backe all the goods, and also brought againe his brother Lot, and his goods, and the women also, and the people.",
        }),

        ("genesis", 14, 17) => Some(Verse {
            content: "And the king of Sodome went out to meete him, (after his returne from the slaughter of Chedorlaomer, and of the Kings that were with him) at the valley of Saueh, which is the Kings dale.",
        }),

        ("genesis", 14, 18) => Some(Verse {
            content: "And Melchizedek King of Salem brought foorth bread and wine: and he was the Priest of the most high God.",
        }),

        ("genesis", 14, 19) => Some(Verse {
            content: "And hee blessed him, and saide; Blessed bee Abram of the most high God, possessour of heauen and earth,",
        }),

        ("genesis", 14, 20) => Some(Verse {
            content: "And blessed bee the most high God, which hath deliuered thine enemies into thy hand: and hee gaue him tithes of all.",
        }),

        ("genesis", 14, 21) => Some(Verse {
            content: "And the King of Sodome said vnto Abram, giue me the persons, and take the goods to thy selfe.",
        }),

        ("genesis", 14, 22) => Some(Verse {
            content: "And Abram said to the King of Sodome, I haue lift vp my hand vnto the LORD, the most high God, the possessour of heauen and earth,",
        }),

        ("genesis", 14, 23) => Some(Verse {
            content: "That I wil not take from a threed euen to a shoe latchet, and that I will not take any thing that is thine, lest thou shouldest say, I haue made Abram rich:",
        }),

        ("genesis", 14, 24) => Some(Verse {
            content: "Saue onely that which the yong men haue eaten, and the portion of the men which went with mee, Aner, Eschol, and Mamre; let them take their portion.",
        }),

        ("genesis", 15, 1) => Some(Verse {
            content: "After these things, the word of the LORD came vnto Abram in a vision, saying; Feare not, Abram: I am thy shield, and thy exceeding great reward.",
        }),

        ("genesis", 15, 2) => Some(Verse {
            content: "And Abram said, Lord GOD, what wilt thou giue me, seeing I goe childlesse? and the steward of my house is this Eliezer of Damascus.",
        }),

        ("genesis", 15, 3) => Some(Verse {
            content: "And Abram said; Behold, to mee thou hast given no seed: and loe, one borne in my house is mine heire.",
        }),

        ("genesis", 15, 4) => Some(Verse {
            content: "And behold, the word of the LORD came vnto him, saying; This shall not be thine heire: but he that shall come foorth out of thy owne bowels, shalbe thine heire.",
        }),

        ("genesis", 15, 5) => Some(Verse {
            content: "And he brought him forth abroad, and said, Looke now towards heauen, and tell the starres, if thou be able to number them. And hee said vnto him, So shall thy seed be.",
        }),

        ("genesis", 15, 6) => Some(Verse {
            content: "And he beleeued in the LORD; and hee counted it to him for righteousnesse.",
        }),

        ("genesis", 15, 7) => Some(Verse {
            content: "And he said vnto him; I am the LORD that brought thee out of Ur of the Caldees, to give thee this land, to inherit it.",
        }),

        ("genesis", 15, 8) => Some(Verse {
            content: "And he said, Lord GOD, whereby shal I know that I shall inherit it?",
        }),

        ("genesis", 15, 9) => Some(Verse {
            content: "And he said vnto him, Take me an heifer of three yeeres old, and a shee goat of three yeeres old, and a ramme of three yeeres old, and a turtle doue, and a yong pigeon.",
        }),

        ("genesis", 15, 10) => Some(Verse {
            content: "And he tooke vnto him all these, and diuided them in the midst, and layd each peece one against another: but the birds diuided he not.",
        }),

        ("genesis", 15, 11) => Some(Verse {
            content: "And when the fowles came downe vpon the carcases, Abram droue them away.",
        }),

        ("genesis", 15, 12) => Some(Verse {
            content: "And when the Sunne was going downe, a deepe sleepe fell vpon Abram: and loe, an horrour of great darkenesse fell vpon him.",
        }),

        ("genesis", 15, 13) => Some(Verse {
            content: "And he said vnto Abram, Know of a surety, that thy seed shalbe a stranger, in a land that is not theirs, and shal serue them, and they shall afflict them foure hundred yeeres.",
        }),

        ("genesis", 15, 14) => Some(Verse {
            content: "And also that nation whom they shall serue, wil I iudge: and afterward shall they come out with great substance.",
        }),

        ("genesis", 15, 15) => Some(Verse {
            content: "And thou shalt goe to thy fathers in peace; thou shalt be buried in a good old age.",
        }),

        ("genesis", 15, 16) => Some(Verse {
            content: "But in the fourth generation they shall come hither againe: for the iniquitie of the Amorites is not yet full.",
        }),

        ("genesis", 15, 17) => Some(Verse {
            content: "And it came to passe that when the Sunne went downe, and it was darke, behold, a smoking furnace, and a burning lampe that passed betweene those pieces.",
        }),

        ("genesis", 15, 18) => Some(Verse {
            content: "In that same day the LORD made a couenant with Abram, saying; Unto thy seed haue I giuen this land from the riuer of Egypt vnto the great riuer, the riuer Euphrates:",
        }),

        ("genesis", 15, 19) => Some(Verse {
            content: "The Kenites, and the Kenizites, and the Kadmonites:",
        }),

        ("genesis", 15, 20) => Some(Verse {
            content: "And the Hittites, and the Perizzites, and the Rephaims,",
        }),

        ("genesis", 15, 21) => Some(Verse {
            content: "And the Hittites, and the Perizzites, and the Rephaims,",
        }),

        ("genesis", 16, 1) => Some(Verse {
            content: "Now Sarai Abrams wife bare him no children: and she had an handmaide, an Egyptian, whose name was Hagar.",
        }),

        ("genesis", 16, 2) => Some(Verse {
            content: "And Sarai said vnto Abram, Behold now, the LORD hath restrained me from bearing: I pray thee go in vnto my maid: it may bee that I may obtaine children by her: and Abram hearkened to the voice of Sarai.",
        }),

        ("genesis", 16, 3) => Some(Verse {
            content: "And Sarai Abrams wife, tooke Hagar her maid, the Egyptian, after Abram had dwelt ten yeeres in the land of Canaan, and gaue her to her husband Abram, to be his wife.",
        }),

        ("genesis", 16, 4) => Some(Verse {
            content: "And he went in vnto Hagar, and she conceiued: And when shee saw that shee had conceiued, her mistresse was despised in her eyes.",
        }),

        ("genesis", 16, 5) => Some(Verse {
            content: "And Sarai said vnto Abram, My wrong be vpon thee: I haue giuen my maid into thy bosome, and when shee saw that she had conceiued, I was despised in her eyes: the LORD iudge betweene me and thee.",
        }),

        ("genesis", 16, 6) => Some(Verse {
            content: "But Abram said vnto Sarai, Behold, thy maid is in thy hand; doe to her as it pleaseth thee. And when Sarai dealt hardly with her, shee fled from her face.",
        }),

        ("genesis", 16, 7) => Some(Verse {
            content: "And the Angel of the LORD found her by a fountaine of water, in the wildernesse, by the fountaine, in the way to Shur:",
        }),

        ("genesis", 16, 8) => Some(Verse {
            content: "And he said, Hagar Sarais maid, whence camest thou? and whither wilt thou goe? And she said, I flee from the face of my mistresse Sarai.",
        }),

        ("genesis", 16, 9) => Some(Verse {
            content: "And the Angel of the LORD said vnto her, Returne to thy mistresse, and submit thy selfe vnder her hands.",
        }),

        ("genesis", 16, 10) => Some(Verse {
            content: "And the Angel of the LORD said vnto her, I will multiply thy seede exceedingly, that it shall not be numbred for multitude.",
        }),

        ("genesis", 16, 11) => Some(Verse {
            content: "And the Angel of the LORD said vnto her, Behold, thou art with child, and shalt beare a sonne, and shalt call his name Ishmael; because the LORD hath heard thy affliction.",
        }),

        ("genesis", 16, 12) => Some(Verse {
            content: "And he will be a wilde man; his hand will be against euery man, and euery mans hand against him: & he shal dwell in the presence of all his brethren.",
        }),

        ("genesis", 16, 13) => Some(Verse {
            content: "And shee called the name of the LORD that spake vnto her, Thou God seest me: for she said, Haue I also here looked after him that seeth me?",
        }),

        ("genesis", 16, 14) => Some(Verse {
            content: "Wherefore the well was called, Beer-lahai-roi: Behold, It is betweene Cadesh and Bered.",
        }),

        ("genesis", 16, 15) => Some(Verse {
            content: "And Hagar bare Abram a sonne: and Abram called his sonnes name, which Hagar bare, Ishmael.",
        }),

        ("genesis", 16, 16) => Some(Verse {
            content: "And Abram was fourescore and sixe yeeres old, when Hagar bare Ishmael to Abram.",
        }),

        ("genesis", 17, 1) => Some(Verse {
            content: "And when Abram was ninetie yeres old and nine, the LORD appeared to Abram, and said vnto him, I am the almightie God, walke before me, and be thou perfect.",
        }),

        ("genesis", 17, 2) => Some(Verse {
            content: "And I wil make my couenant betweene me and thee, and will multiply thee exceedingly.",
        }),

        ("genesis", 17, 3) => Some(Verse {
            content: "And Abram fell on his face, and God talked with him, saying,",
        }),

        ("genesis", 17, 4) => Some(Verse {
            content: "As for me, behold, my couenant is with thee, and thou shalt be a father of many nations.",
        }),

        ("genesis", 17, 5) => Some(Verse {
            content: "Neither shall thy name any more be called Abram, but thy name shall bee Abraham: for a father of many nations haue I made thee.",
        }),

        ("genesis", 17, 6) => Some(Verse {
            content: "And I will make thee exceeding fruitfull, and I will make nations of thee, and Kings shall come out of thee.",
        }),

        ("genesis", 17, 7) => Some(Verse {
            content: "And I will establish my couenant betweene me and thee, and thy seede after thee, in their generations for an euerlasting couenant, to bee a God vnto thee, and to thy seed after thee.",
        }),

        ("genesis", 17, 8) => Some(Verse {
            content: "And I will giue vnto thee, and to thy seed after thee, the land wherein thou art a stranger, all the land of Canaan, for an euerlasting possession, and I will be their God.",
        }),

        ("genesis", 17, 9) => Some(Verse {
            content: "And God said vnto Abraham, Thou shalt keepe my couenant therefore, thou, and thy seede after thee, in their generations.",
        }),

        ("genesis", 17, 10) => Some(Verse {
            content: "This is my couenant, which yee shall keepe betweene me and you, and thy seed after thee: euery man child among you shall be circumcised.",
        }),

        ("genesis", 17, 11) => Some(Verse {
            content: "And ye shall circumcise the flesh of your foreskinne; and it shal be a token of the couenant betwixt me and you.",
        }),

        ("genesis", 17, 12) => Some(Verse {
            content: "And he that is eight dayes olde, shalbe circumcised among you, euery man child in your generations, he that is borne in the house, or bought with money of any stranger, which is not of thy seed.",
        }),

        ("genesis", 17, 13) => Some(Verse {
            content: "He that is borne in thy house, and he that is bought with thy money, must needs be circumcised: and my couenant shall be in your flesh, for an euerlasting couenant.",
        }),

        ("genesis", 17, 14) => Some(Verse {
            content: "And the vncircumcised man-child, whose flesh of his foreskinne is not circumcised, that soule shall be cut off from his people: hee hath broken my couenant.",
        }),

        ("genesis", 17, 15) => Some(Verse {
            content: "And God said vnto Abraham, As for Sarai thy wife, thou shalt not call her name Sarai, but Sarah shall her name be.",
        }),

        ("genesis", 17, 16) => Some(Verse {
            content: "And I will blesse her, and giue thee a sonne also of her: yea I wil blesse her, and she shalbe a mother of nations; Kings of people shall be of her.",
        }),

        ("genesis", 17, 17) => Some(Verse {
            content: "Then Abraham fell vpon his face, and laughed, and said in his heart, Shall a child be borne vnto him that is an hundred yeeres old? and shal Sarah that is ninetie yeeres old, beare?",
        }),

        ("genesis", 17, 18) => Some(Verse {
            content: "And Abraham said vnto God, O that Ishmael might liue before thee.",
        }),

        ("genesis", 17, 19) => Some(Verse {
            content: "And God said, Sarah thy wife shall beare thee a sonne in deede, and thou shalt call his name Isaac: and I will establish my couenant with him, for an euerlasting couenant, and with his seed after him.",
        }),

        ("genesis", 17, 20) => Some(Verse {
            content: "And as for Ishmael, I haue heard thee: behold, I haue blessed him, and will make him fruitfull, and will multiplie him exceedingly: Twelue princes shall he beget, and I will make him a great nation.",
        }),

        ("genesis", 17, 21) => Some(Verse {
            content: "But my couenant wil I establish with Isaac, which Sarah shall beare vnto thee, at this set time, in the next yeere.",
        }),

        ("genesis", 17, 22) => Some(Verse {
            content: "And he left off talking with him, and God went vp from Abraham.",
        }),

        ("genesis", 17, 23) => Some(Verse {
            content: "And Abraham tooke Ishmael his sonne, and all that were borne in his house, and all that were bought with his money, euery male, among the men of Abrahams house, and circumcised the flesh of their foreskinne, in the selfesame day, as God had said vnto him.",
        }),

        ("genesis", 17, 24) => Some(Verse {
            content: "And Abraham was ninety yeeres old and nine, when he was circumcised in the flesh of his foreskinne.",
        }),

        ("genesis", 17, 25) => Some(Verse {
            content: "And Ishmael his sonne was thirteene yeeres old, when he was circumcised in the flesh of his foreskinne.",
        }),

        ("genesis", 17, 26) => Some(Verse {
            content: "In the selfe same day was Abraham circumcised, and Ishmael his sonne.",
        }),

        ("genesis", 17, 27) => Some(Verse {
            content: "And all the men of his house, borne in the house, and bought with money of the stranger, were circumcised with him.",
        }),

        ("genesis", 18, 1) => Some(Verse {
            content: "And the LORD appeared vnto him, in the plaines of Mamre: and he sate in the tent doore, in the heat of the day.",
        }),

        ("genesis", 18, 2) => Some(Verse {
            content: "And he lift vp his eyes and looked, and loe, three men stood by him: and when he saw them, hee ranne to meete them from the tent doore, and bowed himselfe toward the ground,",
        }),

        ("genesis", 18, 3) => Some(Verse {
            content: "And said, My Lord, If now I haue found fauour in thy sight, passe not away, I pray thee, frō thy seruant:",
        }),

        ("genesis", 18, 4) => Some(Verse {
            content: "Let a little water, I pray you, be fetched, and wash your feete, and rest your selues vnder the tree:",
        }),

        ("genesis", 18, 5) => Some(Verse {
            content: "And I will fetch a morsell of bread; and comfort ye your hearts, after that you shall passe on: for therefore are you come to your seruant. And they said; So doe, as thou hast said.",
        }),

        ("genesis", 18, 6) => Some(Verse {
            content: "And Abraham hastened into the tent, vnto Sarah, & said; Make ready quickly three measures of fine meale, knead it, and make cakes vpon the hearth.",
        }),

        ("genesis", 18, 7) => Some(Verse {
            content: "And Abraham ranne vnto the heard, and fetcht a calfe, tender and good, and gaue it vnto a yong man: and he hasted to dresse it.",
        }),

        ("genesis", 18, 8) => Some(Verse {
            content: "And he tooke butter, and milke, and the calfe which he had dressed, and set it before them; and he stood by them vnder the tree: and they did eate.",
        }),

        ("genesis", 18, 9) => Some(Verse {
            content: "And they said vnto him, Where is Sarah thy wife? And he said, Behold, in the tent.",
        }),

        ("genesis", 18, 10) => Some(Verse {
            content: "And he said, I will certainly returne vnto thee according to the time of life; and loe, Sarah thy wife shall haue a sonne. And Sarah heard it in the tent doore, which was behind him.",
        }),

        ("genesis", 18, 11) => Some(Verse {
            content: "Now Abraham and Sarah were old, and well stricken in age: and it ceased to be with Sarah after the maner of women.",
        }),

        ("genesis", 18, 12) => Some(Verse {
            content: "Therefore Sarah laughed within her selfe, saying, After I am waxed old, shall I haue pleasure, my lord being old also?",
        }),

        ("genesis", 18, 13) => Some(Verse {
            content: "And the LORD said vnto Abraham, Wherefore did Sarah laugh, saying; Shall I of a surety beare a childe, which am old?",
        }),

        ("genesis", 18, 14) => Some(Verse {
            content: "Is any thing too hard for the LORD? At the time appointed will I returne vnto thee, according to the time of life, and Sarah shall haue a sonne.",
        }),

        ("genesis", 18, 15) => Some(Verse {
            content: "Then Sarah denied, saying, I laughed not: for she was afraid. And he said, Nay, but thou diddest laugh.",
        }),

        ("genesis", 18, 16) => Some(Verse {
            content: "And the men rose vp from thence, and looked toward Sodome: and Abraham went with them, to bring them on the way.",
        }),

        ("genesis", 18, 17) => Some(Verse {
            content: "And the LORD said, Shall I hide from Abraham that thing which I doe;",
        }),

        ("genesis", 18, 18) => Some(Verse {
            content: "Seeing that Abraham shall surely become a great and mighty nation, and all the nations of the earth shall be blessed in him?",
        }),

        ("genesis", 18, 19) => Some(Verse {
            content: "For I know him, that hee will command his children, and his household after him, and they shall keepe the way of the LORD, to doe iustice and iudgement, that the LORD may bring vpon Abraham, that which hee hath spoken of him.",
        }),

        ("genesis", 18, 20) => Some(Verse {
            content: "And the LORD said, Because the cry of Sodome and Gomorrah is great, and because their sinne is very grieuous:",
        }),

        ("genesis", 18, 21) => Some(Verse {
            content: "I will goe downe now, and see whether they haue done altogether according to the cry of it, which is come vnto me: and if not, I will know.",
        }),

        ("genesis", 18, 22) => Some(Verse {
            content: "And the men turned their faces from thence, and went toward Sodome: but Abraham stood yet before the LORD.",
        }),

        ("genesis", 18, 23) => Some(Verse {
            content: "And Abraham drew neere, and said, Wilt thou also destroy the righteous with the wicked?",
        }),

        ("genesis", 18, 24) => Some(Verse {
            content: "Peraduenture there be fifty righteous within the citie; wilt thou also destroy, and not spare the place for the fiftie righteous, that are therein?",
        }),

        ("genesis", 18, 25) => Some(Verse {
            content: "That be farre from thee, to do after this maner, to slay the righteous with the wicked, and that the righteous should be as the wicked, that be farre from thee: Shall not the Iudge of all the earth doe right?",
        }),

        ("genesis", 18, 26) => Some(Verse {
            content: "And the LORD said, If I find in Sodom fiftie righteous, within the citie, then I will spare all the place for their sakes.",
        }),

        ("genesis", 18, 27) => Some(Verse {
            content: "And Abraham answered, and said, Behold now, I haue taken vpon me to speake vnto the LORD, which am but dust and ashes.",
        }),

        ("genesis", 18, 28) => Some(Verse {
            content: "Peraduenture there shall lacke fiue of the fiftie righteous: wilt thou destroy all the citie for lacke of fiue? And he said, If I find there fourtie and fiue, I will not destroy it.",
        }),

        ("genesis", 18, 29) => Some(Verse {
            content: "And hee spake vnto him yet againe, and said, Peraduenture there shall be fourtie found there: and he said, I will not doe it for fourties sake.",
        }),

        ("genesis", 18, 30) => Some(Verse {
            content: "And he said vnto him, Oh let not the Lord be angry, and I will speake: Peraduenture there shall thirtie bee found there. And he said, I will not doe it, if I find thirtie there.",
        }),

        ("genesis", 18, 31) => Some(Verse {
            content: "And he said, Behold now, I haue taken vpon mee to speake vnto the Lord: Peraduenture there shall bee twenty found there. And he said, I will not destroy it for twenties sake.",
        }),

        ("genesis", 18, 32) => Some(Verse {
            content: "And hee saide, Oh let not the Lord be angry, and I will speake yet but this once: Peraduenture ten shall be found there. And he said, I will not destroy it for tennes sake.",
        }),

        ("genesis", 18, 33) => Some(Verse {
            content: "And the LORD went his way, assoone as hee had left communing with Abraham: and Abraham returned vnto his place.",
        }),

        ("genesis", 19, 1) => Some(Verse {
            content: "And there came two Angels to Sodome at euen, and Lot sate in the gate of Sodome: and Lot seeing them, rose vp to meet them, and he bowed himselfe with his face toward the ground.",
        }),

        ("genesis", 19, 2) => Some(Verse {
            content: "And he said, Beholde now my Lords, turne in, I pray you, into your seruants house, and tarie all night, and wash your feete, and ye shall rise vp early and goe on your wayes. And they said, Nay: but we wil abide in the street all night.",
        }),

        ("genesis", 19, 3) => Some(Verse {
            content: "And he pressed vpon them greatly, and they turned in vnto him, and entred into his house: and he made them a feast, and did bake vnleauened bread, and they did eate.",
        }),

        ("genesis", 19, 4) => Some(Verse {
            content: "But before they lay downe, the men of the citie, euen the men of Sodom, compassed the house round, both old and yong, all the people from euery quarter.",
        }),

        ("genesis", 19, 5) => Some(Verse {
            content: "And they called vnto Lot, and said vnto him, Where are the men which came in to thee this night? bring them out vnto vs, that we may know them.",
        }),

        ("genesis", 19, 6) => Some(Verse {
            content: "And Lot went out at the doore vnto them, & shut the doore after him,",
        }),

        ("genesis", 19, 7) => Some(Verse {
            content: "And said, I pray you, brethren, doe not so wickedly.",
        }),

        ("genesis", 19, 8) => Some(Verse {
            content: "Behold now, I haue two daughters, which haue not knowen man; let mee, I pray you, bring them out vnto you, and doe ye to them, as is good in your eyes: onely vnto these men do nothing: for therefore came they vnder the shadow of my roofe.",
        }),

        ("genesis", 19, 9) => Some(Verse {
            content: "And they said, Stand backe. And they said againe, This one fellow came in to soiourne, and he will needs bee a Iudge: Now wil we deale worse with thee, then with them. And they pressed sore vpon the man, euen Lot, and came neere to breake the doore.",
        }),

        ("genesis", 19, 10) => Some(Verse {
            content: "But the men put forth their hand, and pulled Lot into the house to them, and shut to the doore.",
        }),

        ("genesis", 19, 11) => Some(Verse {
            content: "And they smote the men that were at the doore of the house, with blindnes, both small and great: so that they wearied themselues to finde the doore.",
        }),

        ("genesis", 19, 12) => Some(Verse {
            content: "And the men said vnto Lot, Hast thou here any besides? sonne in law, and thy sonnes, and thy daughters, and whatsoeuer thou hast in the citie, bring them out of this place.",
        }),

        ("genesis", 19, 13) => Some(Verse {
            content: "For we will destroy this place, because the crie of them is waxen great before the face of the LORD: and the LORD hath sent vs to destroy it.",
        }),

        ("genesis", 19, 14) => Some(Verse {
            content: "And Lot went out, and spake vnto his sonnes in law, which married his daughters, and said, Up, get yee out of this place: for the LORD wil destroy this citie: but hee seemed as one that mocked, vnto his sonnes in law.",
        }),

        ("genesis", 19, 15) => Some(Verse {
            content: "And when the morning arose, then the Angels hastened Lot, saying, Arise, take thy wife, & thy two daughters, which are here, lest thou be consumed in the iniquitie of the citie.",
        }),

        ("genesis", 19, 16) => Some(Verse {
            content: "And while he lingred, the men laid hold vpon his hand, and vpon the hand of his wife, and vpon the hand of his two daughters, the LORD being mercifull vnto him: and they brought him forth, and set him without the citie.",
        }),

        ("genesis", 19, 17) => Some(Verse {
            content: "And it came to passe, when they had brought them forth abroad, that he said, Escape for thy life, looke not behind thee, neither stay thou in all the plaine: escape to the mountaine, lest thou bee consumed.",
        }),

        ("genesis", 19, 18) => Some(Verse {
            content: "And Lot said vnto them, Oh not so, my Lord.",
        }),

        ("genesis", 19, 19) => Some(Verse {
            content: "Beholde now, thy seruant hath found grace in thy sight, and thou hast magnified thy mercy, which thou hast shewed vnto me, in sauing my life, and I cannot escape to the mountaine, lest some euill take me, and I die.",
        }),

        ("genesis", 19, 20) => Some(Verse {
            content: "Behold now, this citie is neere to flee vnto, and it is a litle one: Oh let me escape thither, (is it not a litle one?) and my soule shall liue.",
        }),

        ("genesis", 19, 21) => Some(Verse {
            content: "And he said vnto him, See, I haue accepted thee concerning this thing, that I will not ouerthrow this citie, for the which thou hast spoken.",
        }),

        ("genesis", 19, 22) => Some(Verse {
            content: "Haste thee, escape thither: for I cannot doe any thing till thou bee come thither: therefore the name of the citie was called Zoar.",
        }),

        ("genesis", 19, 23) => Some(Verse {
            content: "The sunne was risen vpon the earth, when Lot entred into Zoar.",
        }),

        ("genesis", 19, 24) => Some(Verse {
            content: "Then the LORD rained vpon Sodome & vpon Gomorrah, brimstone and fire, from the LORD out of heauen.",
        }),

        ("genesis", 19, 25) => Some(Verse {
            content: "And he ouerthrew those cities, and all the plaine, and all the inhabitants of the cities, and that which grew vpon the ground.",
        }),

        ("genesis", 19, 26) => Some(Verse {
            content: "But his wife looked backe from behind him, and she became a pillar of salt.",
        }),

        ("genesis", 19, 27) => Some(Verse {
            content: "And Abraham gate vp earely in the morning, to the place, where hee stood before the LORD.",
        }),

        ("genesis", 19, 28) => Some(Verse {
            content: "And he looked toward Sodome and Gomorrah, & toward all the land of the plaine, and beheld, and loe, the smoke of the countrey went vp as the smoke of a furnace.",
        }),

        ("genesis", 19, 29) => Some(Verse {
            content: "And it came to passe, when God destroyed the cities of the plaine, that God remembred Abraham, and sent Lot out of the midst of the ouerthrow, when he ouerthrew the cities, in the which Lot dwelt.",
        }),

        ("genesis", 19, 30) => Some(Verse {
            content: "And Lot went vp out of Zoar, and dwelt in the mountaine, and his two daughters with him: for hee feared to dwell in Zoar, and he dwelt in a caue, he and his two daughters.",
        }),

        ("genesis", 19, 31) => Some(Verse {
            content: "And the first borne saide vnto the yonger, Our father is old, and there is not a man in the earth, to come in vnto vs, after the maner of all the earth.",
        }),

        ("genesis", 19, 32) => Some(Verse {
            content: "Come, let vs make our father drinke wine, and we will lye with him, that we may preserue seed of our father.",
        }),

        ("genesis", 19, 33) => Some(Verse {
            content: "And they made their father drinke wine that night, & the first borne went in, and lay with her father: and he perceiued not, when shee lay downe, nor when she arose.",
        }),

        ("genesis", 19, 34) => Some(Verse {
            content: "And it came to passe on the morrow, that the first borne said vnto the yonger, Behold, I lay yesternight with my father: let vs make him drinke wine this night also, and goe thou in, and lye with him, that we may preserue seed of our father.",
        }),

        ("genesis", 19, 35) => Some(Verse {
            content: "And they made their father drinke wine that night also, and the yonger arose, and lay with him: and he perceiued not, when she lay downe, nor when she arose.",
        }),

        ("genesis", 19, 36) => Some(Verse {
            content: "Thus were both the daughters of Lot with childe by their father.",
        }),

        ("genesis", 19, 37) => Some(Verse {
            content: "And the first borne bare a sonne, and called his name Moab: the same is the father of the Moabites vnto this day.",
        }),

        ("genesis", 19, 38) => Some(Verse {
            content: "And the yonger, she also bare a sonne, and called his name, Ben-ammi: the same is the father of the children of Ammon, vnto this day.",
        }),

        ("genesis", 20, 1) => Some(Verse {
            content: "And Abraham iourneyed from thence, toward the South-Countrey, and dwelled betweene Cadesh and Shur, and soiourned in Gerar.",
        }),

        ("genesis", 20, 2) => Some(Verse {
            content: "And Abraham said of Sarah his wife, She is my sister: And Abimelech King of Gerar sent, and tooke Sarah.",
        }),

        ("genesis", 20, 3) => Some(Verse {
            content: "But God came to Abimelech in a dreame by night, and said to him, Behold, thou art but a dead man, for the woman which thou hast taken: for shee is a mans wife.",
        }),

        ("genesis", 20, 4) => Some(Verse {
            content: "But Abimelech had not come neere her: and he said, LORD, wilt thou slay also a righteous nation?",
        }),

        ("genesis", 20, 5) => Some(Verse {
            content: "Said he not vnto me, She is my sister? and she, euen she herselfe said, Hee is my brother: in the integritie of my heart, and innocencie of my hands haue I done this.",
        }),

        ("genesis", 20, 6) => Some(Verse {
            content: "And God saide vnto him in a dreame, Yea, I know that thou didst this in the integritie of thy heart: for I also withheld thee from sinning against mee, therefore suffered I thee not to touch her.",
        }),

        ("genesis", 20, 7) => Some(Verse {
            content: "Now therefore restore the man his wife: for he is a Prophet, and he shal pray for thee, and thou shalt liue: and if thou restore her not, know thou that thou shalt surely die, thou, and all that are thine.",
        }),

        ("genesis", 20, 8) => Some(Verse {
            content: "Therefore Abimelech rose earely in the morning, and called all his seruants, and told all these things in their eares: and the men were sore afraid.",
        }),

        ("genesis", 20, 9) => Some(Verse {
            content: "Then Abimelech called Abraham, and said vnto him, What hast thou done vnto vs? and what haue I offended thee, that thou hast brought on me, and on my kingdome a great sinne? thou hast done deeds vnto mee that ought not to be done.",
        }),

        ("genesis", 20, 10) => Some(Verse {
            content: "And Abimelech said vnto Abraham, What sawest thou, that thou hast done this thing?",
        }),

        ("genesis", 20, 11) => Some(Verse {
            content: "And Abraham said, Because I thought, Surely the feare of God is not in this place: and they will slay mee for my wiues sake.",
        }),

        ("genesis", 20, 12) => Some(Verse {
            content: "And yet indeed shee is my sister: she is the daughter of my father, but not the daughter of my mother; and shee became my wife.",
        }),

        ("genesis", 20, 13) => Some(Verse {
            content: "And it came to passe when God caused me to wander from my fathers house, that I said vnto her, This is thy kindnesse which thou shalt shew vnto me; at euery place whither wee shall come, say of me, He is my brother.",
        }),

        ("genesis", 20, 14) => Some(Verse {
            content: "And Abimelech tooke sheepe and oxen, and men-seruants, and women seruants, and gaue them vnto Abraham, and restored him Sarah his wife.",
        }),

        ("genesis", 20, 15) => Some(Verse {
            content: "And Abimelech said, Behold, my land is before thee; dwel where it pleaseth thee.",
        }),

        ("genesis", 20, 16) => Some(Verse {
            content: "And vnto Sarah hee said, Behold, I haue giuen thy brother a thousand pieces of siluer: behold, he is to thee a couering of the eyes, vnto all that are with thee, and with all other: thus shee was reproued.",
        }),

        ("genesis", 20, 17) => Some(Verse {
            content: "So Abraham prayed vnto God: and God healed Abimelech, and his wife, and his maid-seruants, and they bare children.",
        }),

        ("genesis", 20, 18) => Some(Verse {
            content: "For the LORD had fast closed vp all the wombes of the house of Abimelech, because of Sarah Abrahams wife.",
        }),

        ("genesis", 21, 1) => Some(Verse {
            content: "And the LORD visited Sarah as he had said, and the LORD did vnto Sarah as he had spoken.",
        }),

        ("genesis", 21, 2) => Some(Verse {
            content: "For Sarah conceiued, and bare Abraham a sonne in his old age, at the set time, of which God had spoken to him.",
        }),

        ("genesis", 21, 3) => Some(Verse {
            content: "And Abraham called the name of his sonne, that was borne vnto him, whom Sarah bare to him, Isaac.",
        }),

        ("genesis", 21, 4) => Some(Verse {
            content: "And Abraham circumcised his sonne Isaac, being eight dayes old, as God had commanded him.",
        }),

        ("genesis", 21, 5) => Some(Verse {
            content: "And Abraham was an hundred yeeres old, when his sonne Isaac was borne vnto him.",
        }),

        ("genesis", 21, 6) => Some(Verse {
            content: "And Sarah said, God hath made me to laugh, so that all that heare, will laugh with me.",
        }),

        ("genesis", 21, 7) => Some(Verse {
            content: "And she said, Who would haue said vnto Abraham, that Sarah should haue giuen children sucke? for I haue borne him a sonne in his old age.",
        }),

        ("genesis", 21, 8) => Some(Verse {
            content: "And the child grew, and was weaned: and Abraham made a great feast, the same day that Isaac was weaned.",
        }),

        ("genesis", 21, 9) => Some(Verse {
            content: "And Sarah saw the sonne of Hagar the Egyptian, which shee had borne vnto Abraham, mocking.",
        }),

        ("genesis", 21, 10) => Some(Verse {
            content: "Wherfore she said vnto Abraham, Cast out this bond woman, and her sonne: for the sonne of this bond woman shall not be heire with my sonne, euen with Isaac.",
        }),

        ("genesis", 21, 11) => Some(Verse {
            content: "And the thing was very grieuous in Abrahams sight, because of his sonne.",
        }),

        ("genesis", 21, 12) => Some(Verse {
            content: "And God said vnto Abraham, Let it not be grieuous in thy sight, because of the lad, and because of thy bond woman. In all that Sarah hath said vnto thee, hearken vnto her voice: for in Isaac shall thy seed be called.",
        }),

        ("genesis", 21, 13) => Some(Verse {
            content: "And also, of the sonne of the bond woman will I make a nation, because he is thy seed.",
        }),

        ("genesis", 21, 14) => Some(Verse {
            content: "And Abraham rose vp earely in the morning, and tooke bread, and a bottle of water, and gaue it vnto Hagar, (putting it on her shoulder,) and the child, and sent her away: and shee departed, and wandered in the wildernesse of Beer-sheba.",
        }),

        ("genesis", 21, 15) => Some(Verse {
            content: "And the water was spent in the bottle, and shee cast the child vnder one of the shrubs.",
        }),

        ("genesis", 21, 16) => Some(Verse {
            content: "And she went, and sate her downe ouer against him, a good way off, as it were a bow shoot: for she said, Let me not see the death of the child. And shee sate ouer against him, and lift vp her voice, and wept.",
        }),

        ("genesis", 21, 17) => Some(Verse {
            content: "And God heard the voice of the lad, and the Angel of God called to Hagar out of heauen, and said vnto her, What aileth thee, Hagar? feare not: for God hath heard the voice of the ladde, where he is.",
        }),

        ("genesis", 21, 18) => Some(Verse {
            content: "Arise, lift vp the lad, and hold him in thine hand: for I will make him a great nation.",
        }),

        ("genesis", 21, 19) => Some(Verse {
            content: "And God opened her eyes, and she saw a well of water, and shee went, and filled the bottle with water, and gaue the lad drinke.",
        }),

        ("genesis", 21, 20) => Some(Verse {
            content: "And God was with the lad, and he grew, and dwelt in the wildernesse, and became an archer.",
        }),

        ("genesis", 21, 21) => Some(Verse {
            content: "And hee dwelt in the wildernesse of Paran: and his mother tooke him a wife out of the land of Egypt.",
        }),

        ("genesis", 21, 22) => Some(Verse {
            content: "And it came to passe at that time, that Abimelech and Phichol the chiefe captaine of his hoste spake vnto Abraham, saying, God is with thee in all that thou doest.",
        }),

        ("genesis", 21, 23) => Some(Verse {
            content: "Now therefore sweare vnto mee here by God, that thou wilt not deale falsly with me, nor with my sonne, nor with my sonnes sonne: but according to the kindnesse that I haue done vnto thee, thou shalt doe vnto me, and to the land wherein thou hast soiourned.",
        }),

        ("genesis", 21, 24) => Some(Verse {
            content: "And Abraham saide, I will sweare.",
        }),

        ("genesis", 21, 25) => Some(Verse {
            content: "And Abraham reproued Abimelech, because of a well of water, which Abimelechs seruants had violently taken away.",
        }),

        ("genesis", 21, 26) => Some(Verse {
            content: "And Abimelech saide, I wote not who hath done this thing: neither didst thou tell me, neither yet heard I of it, but to day.",
        }),

        ("genesis", 21, 27) => Some(Verse {
            content: "And Abraham tooke sheepe and oxen, and gaue them vnto Abimelech: and both of them made a couenant.",
        }),

        ("genesis", 21, 28) => Some(Verse {
            content: "And Abraham set seuen ewe lambes of the flocke by themselues.",
        }),

        ("genesis", 21, 29) => Some(Verse {
            content: "And Abimelech said vnto Abraham, What meane these seuen ewe lambes, which thou hast set by themselues?",
        }),

        ("genesis", 21, 30) => Some(Verse {
            content: "And he said, For these seuen ewe lambs shalt thou take of my hand, that they may be a witnesse vnto me, that I haue digged this well.",
        }),

        ("genesis", 21, 31) => Some(Verse {
            content: "Wherefore he called that place, Beer-sheba: because there they sware both of them.",
        }),

        ("genesis", 21, 32) => Some(Verse {
            content: "Thus they made a couenant at Beeer-sheba: then Abimelech rose vp, and Phichol the chiefe captaine of his hoste, and they returned into the land of the Philistines.",
        }),

        ("genesis", 21, 33) => Some(Verse {
            content: "And Abraham planted a groue in Beer-sheba, and called there on the Name of the LORD, the euerlasting God.",
        }),

        ("genesis", 21, 34) => Some(Verse {
            content: "And Abraham soiourned in the Philistines land, many dayes.",
        }),

        ("genesis", 22, 1) => Some(Verse {
            content: "And it came to passe after these things, that God did tempt Abraham, and said vnto him, Abraham. And hee said, Beholde, heere I am.",
        }),

        ("genesis", 22, 2) => Some(Verse {
            content: "And he said, Take now thy sonne, thine onely sonne Isaac, whom thou louest, and get thee into the land of Moriah: and offer him there for a burnt offering vpon one of the Mountaines which I will tell thee of.",
        }),

        ("genesis", 22, 3) => Some(Verse {
            content: "And Abraham rose vp earely in the morning, and sadled his asse, and tooke two of his yong men with him, and Isaac his sonne, and claue the wood for the burnt offering, and rose vp, and went vnto the place of which God had told him.",
        }),

        ("genesis", 22, 4) => Some(Verse {
            content: "Then on the third day Abraham lift vp his eyes, and saw the place afarre off.",
        }),

        ("genesis", 22, 5) => Some(Verse {
            content: "And Abraham said vnto his yong men, Abide you here with the asse, and I and the lad will goe yonder and worship, and come againe to you.",
        }),

        ("genesis", 22, 6) => Some(Verse {
            content: "And Abraham tooke the wood of the burnt offering, and layd it vpon Isaac his sonne: and he tooke the fire in his hand, and a knife: and they went both of them together.",
        }),

        ("genesis", 22, 7) => Some(Verse {
            content: "And Isaac spake vnto Abraham his father, and said, My father: and he said, Here am I, my sonne. And hee said, Behold the fire and wood: but where is the lambe for a burnt offring?",
        }),

        ("genesis", 22, 8) => Some(Verse {
            content: "And Abraham said, My sonne, God will prouide himselfe a lambe for a burnt offering: so they went both of them together.",
        }),

        ("genesis", 22, 9) => Some(Verse {
            content: "And they came to the place which God had tolde him of, and Abraham built an Altar there, and layd the wood in order, and bound Isaac his sonne, and layde him on the Altar vpon the wood.",
        }),

        ("genesis", 22, 10) => Some(Verse {
            content: "And Abraham stretched foorth his hand, and tooke the knife to slay his sonne.",
        }),

        ("genesis", 22, 11) => Some(Verse {
            content: "And the Angel of the LORD called vnto him out of heauen, and said, Abraham, Abraham. And he said, Here am I.",
        }),

        ("genesis", 22, 12) => Some(Verse {
            content: "And he said, Lay not thine hand vpon the lad, neither do thou any thing vnto him: for now I know that thou fearest God, seeing thou hast not withhelde thy sonne, thine onely sonne from mee.",
        }),

        ("genesis", 22, 13) => Some(Verse {
            content: "And Abraham lifted vp his eyes, and looked, and beholde, behinde him a Ramme caught in a thicket by his hornes: And Abraham went and tooke the Ramme, and offered him vp for a burnt offering, in the stead of his sonne.",
        }),

        ("genesis", 22, 14) => Some(Verse {
            content: "And Abraham called the name of that place Iehouah-ijreh, as it is said to this day, In the Mount of the LORD it shalbe seene.",
        }),

        ("genesis", 22, 15) => Some(Verse {
            content: "And the Angel of the LORD called vnto Abraham out of heauen the second time,",
        }),

        ("genesis", 22, 16) => Some(Verse {
            content: "And said, By my selfe haue I sworne, saith the LORD, for because thou hast done this thing, and hast not withheld thy sonne, thine onely sonne,",
        }),

        ("genesis", 22, 17) => Some(Verse {
            content: "That in blessing I will blesse thee, and in multiplying, I will multiply thy seed as the starres of the heauen, and as the sand which is vpon the sea shore, and thy seed shall possesse the gate of his enemies.",
        }),

        ("genesis", 22, 18) => Some(Verse {
            content: "And in thy seed shall all the nations of the earth be blessed, because thou hast obeyed my voice.",
        }),

        ("genesis", 22, 19) => Some(Verse {
            content: "So Abraham returned vnto his yong men, and they rose vp, and went together to Beer-sheba, and Abraham dwelt at Beer-sheba.",
        }),

        ("genesis", 22, 20) => Some(Verse {
            content: "And it came to passe after these things, that it was told Abraham, saying, Behold Milcah, shee hath also borne children vnto thy brother Nahor,",
        }),

        ("genesis", 22, 21) => Some(Verse {
            content: "Huz his first borne, and Buz his brother, and Kemuel the father of Aram,",
        }),

        ("genesis", 22, 22) => Some(Verse {
            content: "And Chesed, and Hazo, and Pildash, and Iidlaph, and Bethuel.",
        }),

        ("genesis", 22, 23) => Some(Verse {
            content: "And Bethuel begate Rebekah: these eight Milcah did beare to Nahor, Abrahams brother.",
        }),

        ("genesis", 22, 24) => Some(Verse {
            content: "And his concubine whose name was Reumah, she bare also Tebah, and Gaham, and Thahash, and Maachah.",
        }),

        ("genesis", 23, 1) => Some(Verse {
            content: "And Sarah was an hundred and seuen and twenty yeeres olde: these were the yeeres of the life of Sarah.",
        }),

        ("genesis", 23, 2) => Some(Verse {
            content: "And Sarah died in Kiriath arba, the same is Hebron in the land of Canaan: And Abraham came to mourne for Sarah, and to weepe for her.",
        }),

        ("genesis", 23, 3) => Some(Verse {
            content: "And Abraham stood vp from before his dead, & spake vnto the sonnes of Heth, saying,",
        }),

        ("genesis", 23, 4) => Some(Verse {
            content: "I am a stranger and a soiourner with you: giue me a possession of a burying place with you, that I may bury my dead out of my sight.",
        }),

        ("genesis", 23, 5) => Some(Verse {
            content: "And the children of Heth answered Abraham, saying vnto him,",
        }),

        ("genesis", 23, 6) => Some(Verse {
            content: "Heare vs, my Lord, thou art a mightie Prince amongst vs: in the choise of our sepulchres bury thy dead: none of vs shall withhold from thee his sepulchre, but that thou mayest bury thy dead.",
        }),

        ("genesis", 23, 7) => Some(Verse {
            content: "And Abraham stood vp and bowed himselfe to the people of the land, euen to the children of Heth.",
        }),

        ("genesis", 23, 8) => Some(Verse {
            content: "And hee communed with them, saying, if it be your mind that I should bury my dead out of my sight, heare me, and entreat for me to Ephron the sonne of Zohar:",
        }),

        ("genesis", 23, 9) => Some(Verse {
            content: "That he may giue me the caue of Machpelah, which he hath, which is in the end of his field: for as much money as it is worth he shall giue it mee, for a possession of a burying place amongst you.",
        }),

        ("genesis", 23, 10) => Some(Verse {
            content: "And Ephron dwelt amongst the children of Heth. And Ephron the Hittite answered Abraham in the audience of the children of Heth, euen of all that went in at the gates of his citie, saying,",
        }),

        ("genesis", 23, 11) => Some(Verse {
            content: "Nay, my lord, heare mee: the field giue I thee, and the caue that is therein, I giue it thee, in the presence of the sonnes of my people giue I it thee: bury thy dead.",
        }),

        ("genesis", 23, 12) => Some(Verse {
            content: "And Abraham bowed downe himselfe before the people of the land.",
        }),

        ("genesis", 23, 13) => Some(Verse {
            content: "And he spake vnto Ephron in the audience of the people of the land, saying, But if thou wilt giue it, I pray thee, heare mee: I will giue thee money for the field: take it of me, and I will bury my dead there.",
        }),

        ("genesis", 23, 14) => Some(Verse {
            content: "And Ephron answered Abraham, saying vnto him,",
        }),

        ("genesis", 23, 15) => Some(Verse {
            content: "My lord, hearken vnto mee: the land is worth foure hundred shekels of siluer: what is that betwixt mee and thee? bury therefore thy dead.",
        }),

        ("genesis", 23, 16) => Some(Verse {
            content: "And Abraham hearkened vnto Ephron, and Abraham weighed to Ephron the siluer, which he had named, in the audience of the sonnes of Heth, foure hundred shekels of siluer, currant money with the merchant.",
        }),

        ("genesis", 23, 17) => Some(Verse {
            content: "And the field of Ephron which was in Machpelah, which was before Mamre, the fielde and the caue which was therein, and all the trees that were in the field, that were in all the borders round about, were made sure",
        }),

        ("genesis", 23, 18) => Some(Verse {
            content: "Unto Abraham for a possession in the presence of the children of Heth, before all that went in at the gates of his Citie.",
        }),

        ("genesis", 23, 19) => Some(Verse {
            content: "And after this Abraham buried Sarah his wife in the caue of the field of Machpelah, before Mamre: the same is Hebron in the land of Canaan.",
        }),

        ("genesis", 23, 20) => Some(Verse {
            content: "And the field, and the caue that is therein, were made sure vnto Abraham, for a possession of a burying place, by the sonnes of Heth.",
        }),

        ("genesis", 24, 1) => Some(Verse {
            content: "And Abraham was olde and well stricken in age: And the LORD had blessed Abraham in all things.",
        }),

        ("genesis", 24, 2) => Some(Verse {
            content: "And Abraham said vnto his eldest seruant of his house, that ruled ouer all that he had, Put, I pray thee, thy hand vnder my thigh:",
        }),

        ("genesis", 24, 3) => Some(Verse {
            content: "And I will make thee sweare by the LORD the God of heauen, and the God of the earth, that thou shalt not take a wife vnto my sonne of the daughters of the Canaanites amongst whom I dwell.",
        }),

        ("genesis", 24, 4) => Some(Verse {
            content: "But thou shalt go vnto my countrey, and to my kinred, and take a wife vnto my sonne Isaac.",
        }),

        ("genesis", 24, 5) => Some(Verse {
            content: "And the seruant said vnto him, Peraduenture the woman will not bee willing to follow mee vnto this land: must I needes bring thy sonne againe, vnto the land from whence thou camest?",
        }),

        ("genesis", 24, 6) => Some(Verse {
            content: "And Abraham said vnto him, Beware thou, that thou bring not my sonne thither againe.",
        }),

        ("genesis", 24, 7) => Some(Verse {
            content: "The LORD God of heauen which tooke mee from my fathers house, and from the land of my kindred, and which spake vnto mee, and that sware vnto me, saying, Unto thy seed will I giue this land, he shall send his Angel before thee, and thou shalt take a wife vnto my sonne from thence.",
        }),

        ("genesis", 24, 8) => Some(Verse {
            content: "And if the woman wil not be willing to follow thee, then thou shalt bee cleare from this my othe: onely bring not my sonne thither againe.",
        }),

        ("genesis", 24, 9) => Some(Verse {
            content: "And the seruant put his hand vnder the thigh of Abraham his master, and sware to him concerning that matter.",
        }),

        ("genesis", 24, 10) => Some(Verse {
            content: "And the seruant tooke ten camels, of the camels of his master, and departed, ( for all the goods of his master were in his hand) and he arose, and went to Mesopotamia, vnto the citie of Nahor.",
        }),

        ("genesis", 24, 11) => Some(Verse {
            content: "And he made his camels to kneele downe without the citie, by a well of water, at the time of the euening, euen the time that women goe out to draw water.",
        }),

        ("genesis", 24, 12) => Some(Verse {
            content: "And he said, O LORD, God of my master Abraham, I pray thee send me good speed this day, and shew kindnesse vnto my master Abraham.",
        }),

        ("genesis", 24, 13) => Some(Verse {
            content: "Behold, I stand here by the well of water; and the daughters of the men of the Citie come out to draw water:",
        }),

        ("genesis", 24, 14) => Some(Verse {
            content: "And let it come to passe, that the damsell to whom I shall say, Let downe thy pitcher, I pray thee, that I may drinke, and she shall say, Drinke, and I will giue thy camels drinke also; let the same be shee that thou hast appointed for thy seruant Isaac: and thereby shall I know that thou hast shewed kindnesse vnto my master.",
        }),

        ("genesis", 24, 15) => Some(Verse {
            content: "And it came to passe before hee had done speaking, that behold, Rebekah came out, who was borne to Bethuel, sonne of Milcah, the wife of Nahor Abrahams brother, with her pitcher vpon her shoulder.",
        }),

        ("genesis", 24, 16) => Some(Verse {
            content: "And the damsell was very faire to looke vpon, a virgine, neither had any man knowen her; and shee went downe to the wel, and filled her pitcher, and came vp.",
        }),

        ("genesis", 24, 17) => Some(Verse {
            content: "And the seruant ranne to meete her, and said, Let mee (I pray thee) drinke a little water of thy pitcher.",
        }),

        ("genesis", 24, 18) => Some(Verse {
            content: "And she said, Drinke, my lord: and she hasted, and let downe her pitcher vpon her hand, and gaue him drinke.",
        }),

        ("genesis", 24, 19) => Some(Verse {
            content: "And when shee had done giuing him drinke, she said, I will draw water for thy camels also, vntill they haue done drinking.",
        }),

        ("genesis", 24, 20) => Some(Verse {
            content: "And she hasted and emptied her pitcher into the trough, and ranne againe vnto the well to draw water, and drew for all his camels.",
        }),

        ("genesis", 24, 21) => Some(Verse {
            content: "And the man wondering at her, helde his peace, to wit, whether the LORD had made his iourney prosperous, or not.",
        }),

        ("genesis", 24, 22) => Some(Verse {
            content: "And it came to passe as the camels had done drinking, that the man tooke a golden eare-ring, of halfe a shekel weight, & two bracelets for her handes, of ten shekels weight of gold,",
        }),

        ("genesis", 24, 23) => Some(Verse {
            content: "And said, whose daughter art thou? tell mee, I pray thee: is there roome in thy fathers house for vs to lodge in?",
        }),

        ("genesis", 24, 24) => Some(Verse {
            content: "And she said vnto him, I am the daughter of Bethuel the sonne of Milcah, which she bare vnto Nahor:",
        }),

        ("genesis", 24, 25) => Some(Verse {
            content: "She said moreouer vnto him, We haue both straw & prouender ynough, and roome to lodge in.",
        }),

        ("genesis", 24, 26) => Some(Verse {
            content: "And the man bowed downe his head, and worshipped the LORD.",
        }),

        ("genesis", 24, 27) => Some(Verse {
            content: "And hee saide, Blessed bee the LORD God of my master Abraham, who hath not left destitute my master of his mercy, and his trueth: I being in the way, the LORD led me to the house of my masters brethren.",
        }),

        ("genesis", 24, 28) => Some(Verse {
            content: "And the damsell ranne, and told them of her mothers house, these things.",
        }),

        ("genesis", 24, 29) => Some(Verse {
            content: "And Rebekah had a brother, and his name was Laban: and Laban ranne out vnto the man, vnto the well.",
        }),

        ("genesis", 24, 30) => Some(Verse {
            content: "And it came to passe when he saw the eare-ring, and bracelets vpon his sisters hands, and when hee heard the wordes of Rebekah his sister, saying, Thus spake the man vnto me, that he came vnto the man; and behold, hee stood by the camels, at the well.",
        }),

        ("genesis", 24, 31) => Some(Verse {
            content: "And he said, Come in, thou blessed of the LORD, wherefore standest thou without? for I haue prepared the house, and roome for the camels.",
        }),

        ("genesis", 24, 32) => Some(Verse {
            content: "And the man came into the house: and he vngirded his camels, and gaue straw and prouender for the camels, and water to wash his feet, and the mens feet that were with him.",
        }),

        ("genesis", 24, 33) => Some(Verse {
            content: "And there was set meat before him to eate: but he said, I will not eate, vntill I haue tolde mine errand. And hee said, Speake on.",
        }),

        ("genesis", 24, 34) => Some(Verse {
            content: "And he said, I am Abrahams seruant.",
        }),

        ("genesis", 24, 35) => Some(Verse {
            content: "And the LORD hath blessed my master greatly, and hee is become great: and hee hath giuen him flocks, and heards, and siluer, and gold, and men seruants, and mayd seruants, and camels, and asses.",
        }),

        ("genesis", 24, 36) => Some(Verse {
            content: "And Sarah my masters wife bare a sonne to my master when shee was old: and vnto him hath hee giuen all that he hath.",
        }),

        ("genesis", 24, 37) => Some(Verse {
            content: "And my master made me sweare, saying, Thou shalt not take a wife to my sonne, of the daughters of the Canaanites, in whose land I dwell:",
        }),

        ("genesis", 24, 38) => Some(Verse {
            content: "But thou shalt goe vnto my fathers house, and to my kinred, and take a wife vnto my sonne.",
        }),

        ("genesis", 24, 39) => Some(Verse {
            content: "And I said vnto my master, Peraduenture the woman will not followe me.",
        }),

        ("genesis", 24, 40) => Some(Verse {
            content: "And hee saide vnto me, The LORD, before whom I walke, will send his Angel with thee, and prosper thy way: and thou shalt take a wife for my sonne, of my kinred, and of my fathers house.",
        }),

        ("genesis", 24, 41) => Some(Verse {
            content: "Then shalt thou bee cleare from this my oath, when thou commest to my kinred, and if they giue not thee one, thou shalt be cleare from my oath.",
        }),

        ("genesis", 24, 42) => Some(Verse {
            content: "And I came this day vnto the well, and said, O LORD God of my master Abraham, if now thou doe prosper my way, which I goe:",
        }),

        ("genesis", 24, 43) => Some(Verse {
            content: "Behold, I stand by the well of water; and it shall come to passe, that when the virgine commeth foorth to draw water, and I say to her, Giue me, I pray thee, a litle water of thy pitcher to drinke;",
        }),

        ("genesis", 24, 44) => Some(Verse {
            content: "And she say to me, Both drinke thou, and I will also draw for thy camels: let the same be the woman, whō the LORD hath appointed out for my masters sonne.",
        }),

        ("genesis", 24, 45) => Some(Verse {
            content: "And before I had done speaking in mine heart, behold, Rebekah came forth, with her pitcher on her shoulder; and she went downe vnto the well, and drew water: and I said vnto her, Let me drinke, I pray thee.",
        }),

        ("genesis", 24, 46) => Some(Verse {
            content: "And she made haste, & let downe her pitcher from her shoulder, and saide, Drinke, and I will giue thy camels drinke also: so I dranke, and she made the camels drinke also.",
        }),

        ("genesis", 24, 47) => Some(Verse {
            content: "And I asked her, and said, whose daughter art thou? and she said, The daughter of Bethuel, Nahors sonne, whom Milcah bare vnto him: and I put the earering vpon her face, and the bracelets vpon her hands.",
        }),

        ("genesis", 24, 48) => Some(Verse {
            content: "And I bowed downe my head, and worshipped the LORD, and blessed the LORD God of my master Abraham, which had led mee in the right way to take my masters brothers daughter vnto his sonne.",
        }),

        ("genesis", 24, 49) => Some(Verse {
            content: "And now if you wil deale kindly and truely with my master, tell me: and if not, tell me, that I may turne to the right hand, or to the left.",
        }),

        ("genesis", 24, 50) => Some(Verse {
            content: "Then Laban and Bethuel answered and said, The thing proceedeth from the LORD: we cannot speake vnto thee bad or good.",
        }),

        ("genesis", 24, 51) => Some(Verse {
            content: "Behold, Rebekah is before thee, take her, and goe, and let her be thy masters sonnes wife, as the LORD hath spoken.",
        }),

        ("genesis", 24, 52) => Some(Verse {
            content: "And it came to passe, that when Abrahams seruant heard their words, he worshipped the LORD, bowing himselfe to the earth.",
        }),

        ("genesis", 24, 53) => Some(Verse {
            content: "And the seruant brought foorth iewels of siluer, and iewels of gold, and raiment, and gaue them to Rebekah: He gaue also to her brother, and to her mother precious things.",
        }),

        ("genesis", 24, 54) => Some(Verse {
            content: "And they did eate and drinke, he and the men that were with him, and taried all night, and they rose vp in the morning, and he said, Send me away vnto my master.",
        }),

        ("genesis", 24, 55) => Some(Verse {
            content: "And her brother and her mother said, Let the damsell abide with vs a few dayes, at the least ten; after that, she shall goe.",
        }),

        ("genesis", 24, 56) => Some(Verse {
            content: "And he said vnto them, Hinder me not, seeing the LORD hath prospered my way: send me away, that I may goe to my master.",
        }),

        ("genesis", 24, 57) => Some(Verse {
            content: "And they said, wee will call the Damsell, and enquire at her mouth.",
        }),

        ("genesis", 24, 58) => Some(Verse {
            content: "And they called Rebekah, and said vnto her, Wilt thou go with this man? and she said, I will goe.",
        }),

        ("genesis", 24, 59) => Some(Verse {
            content: "And they sent away Rebekah their sister, and her nurse, and Abrahams seruant, and his men.",
        }),

        ("genesis", 24, 60) => Some(Verse {
            content: "And they blessed Rebekah, and said vnto her, Thou art our sister, bee thou the mother of thousands of millions, and let thy seed possesse the gate of those which hate them.",
        }),

        ("genesis", 24, 61) => Some(Verse {
            content: "And Rebekah arose, and her damsels, & they rode vpon the camels, and followed the man: and the seruant tooke Rebekah, and went his way.",
        }),

        ("genesis", 24, 62) => Some(Verse {
            content: "And Isaac came from the way of the well Lahai-roi, for he dwelt in the South countrey.",
        }),

        ("genesis", 24, 63) => Some(Verse {
            content: "And Isaac went out, to meditate in the field, at the euentide: and hee lift vp his eyes, and saw, and behold, the camels were comming.",
        }),

        ("genesis", 24, 64) => Some(Verse {
            content: "And Rebekah lift vp her eyes, and when she saw Isaac, she lighted off the camel.",
        }),

        ("genesis", 24, 65) => Some(Verse {
            content: "For she had said vnto the seruant, What man is this that walketh in the field to meet vs? and the seruant had said, It is my master: therefore shee tooke a vaile and couered her selfe.",
        }),

        ("genesis", 24, 66) => Some(Verse {
            content: "And the seruant tolde Isaac all things that he had done.",
        }),

        ("genesis", 24, 67) => Some(Verse {
            content: "And Isaac brought her into his mother Sarahs tent, and tooke Rebekah, and she became his wife, and he loued her: and Isaac was comforted after his mothers death.",
        }),

        ("genesis", 25, 1) => Some(Verse {
            content: "Then againe Abraham tooke a wife, & her name was Keturah.",
        }),

        ("genesis", 25, 2) => Some(Verse {
            content: "And shee bare him Zimran, and Iokshan, and Medan, and Midian, and Ishbak, and Shuah.",
        }),

        ("genesis", 25, 3) => Some(Verse {
            content: "And Iokshan begat Sheba, and Dedan. And the sonnes of Dedan were Asshurim, and Letushim, and Leummim.",
        }),

        ("genesis", 25, 4) => Some(Verse {
            content: "And the sonnes of Midian, Ephah, and Epher, and Hanoch, and Abida, and Eldaah: all these were the children of Keturah.",
        }),

        ("genesis", 25, 5) => Some(Verse {
            content: "And Abraham gaue all that he had, vnto Isaac.",
        }),

        ("genesis", 25, 6) => Some(Verse {
            content: "But vnto the sonnes of the concubines which Abraham had, Abraham gaue gifts, and sent them away from Isaac his sonne (while he yet liued) Eastward, vnto the East country.",
        }),

        ("genesis", 25, 7) => Some(Verse {
            content: "And these are the dayes of the yeres of Abrahams life which he liued; an hundred, threescore & fifteene yeeres.",
        }),

        ("genesis", 25, 8) => Some(Verse {
            content: "Then Abraham gaue vp the ghost, and died in a good old age, an old man, and full of yeeres, and was gathered to his people.",
        }),

        ("genesis", 25, 9) => Some(Verse {
            content: "And his sonnes Isaac and Ishmael buried him in the caue of Machpelah, in the field of Ephron the sonne of Zohar the Hittite, which is before Mamre;",
        }),

        ("genesis", 25, 10) => Some(Verse {
            content: "The field which Abraham purchased of the sonnes of Heth: there was Abraham buried, and Sarah his wife.",
        }),

        ("genesis", 25, 11) => Some(Verse {
            content: "And it came to passe after the death of Abraham, that God blessed his sonne Isaac, and Isaac dwelt by the well Lahai-roi.",
        }),

        ("genesis", 25, 12) => Some(Verse {
            content: "Now these are the generations of Ishmael Abrahams sonne, whom Hagar the Egyptian Sarahs handmayd, bare vnto Abraham:",
        }),

        ("genesis", 25, 13) => Some(Verse {
            content: "And these are the names of the sonnes of Ishmael, by their names, according to their generations; The first borne of Ishmael, Nebaioth, and Kedar, and Adbeel, and Mibsam,",
        }),

        ("genesis", 25, 14) => Some(Verse {
            content: "And Mishma, and Dumah, and Massa,",
        }),

        ("genesis", 25, 15) => Some(Verse {
            content: "Hadar, and Tema, Ietur, Naphish, and Kedemah.",
        }),

        ("genesis", 25, 16) => Some(Verse {
            content: "These are the sonnes of Ishmael, and these are their names, by their townes and by their castels; twelue princes according to their nations.",
        }),

        ("genesis", 25, 17) => Some(Verse {
            content: "And these are the yeeres of the life of Ishmael; an hundred and thirty and seuen yeeres: and he gaue vp the ghost and died, and was gathered vnto his people.",
        }),

        ("genesis", 25, 18) => Some(Verse {
            content: "And they dwelt from Hauilah vnto Shur, that is before Egypt, as thou goest towards Assyria: and hee died in the presence of all his brethren.",
        }),

        ("genesis", 25, 19) => Some(Verse {
            content: "And these are the generations of Isaac, Abrahams sonne: Abraham begate Isaac.",
        }),

        ("genesis", 25, 20) => Some(Verse {
            content: "And Isaac was fortie yeeres old when hee tooke Rebekah to wife, the daughter of Bethuel the Syrian of Padan Aram, the sister to Laban the Syrian.",
        }),

        ("genesis", 25, 21) => Some(Verse {
            content: "And Isaac intreated the LORD for his wife, because she was barren: and the LORD was intreated of him, and Rebekah his wife conceiued.",
        }),

        ("genesis", 25, 22) => Some(Verse {
            content: "And the children struggled together within her; and she said, If it be so, why am I thus? and shee went to enquire of the LORD.",
        }),

        ("genesis", 25, 23) => Some(Verse {
            content: "And the LORD said vnto her, Two nations are in thy wombe, and two maner of people shall be separated from thy bowels: and the one people shalbe stronger then the other people: and the elder shall serue the yonger.",
        }),

        ("genesis", 25, 24) => Some(Verse {
            content: "And when her dayes to be deliuered were fulfilled, behold, there were twinnes in her wombe.",
        }),

        ("genesis", 25, 25) => Some(Verse {
            content: "And the first came out red, all ouer like an hairy garment: and they called his name, Esau.",
        }),

        ("genesis", 25, 26) => Some(Verse {
            content: "And after that came his brother out, and his hand tooke holde on Esaus heele; and his name was called Iacob: and Isaac was threescore yeres old, when shee bare them.",
        }),

        ("genesis", 25, 27) => Some(Verse {
            content: "And the boyes grew; and Esau was a cunning hunter, a man of the fielde: and Iacob was a plaine man, dwelling in tents.",
        }),

        ("genesis", 25, 28) => Some(Verse {
            content: "And Isaac loued Esau, because he did eate of his venison: but Rebekah loued Iacob.",
        }),

        ("genesis", 25, 29) => Some(Verse {
            content: "And Iacob sod pottage: and Esau came from the field, and hee was faint.",
        }),

        ("genesis", 25, 30) => Some(Verse {
            content: "And Esau said to Iacob, Feed me, I pray thee, with that same red pottage: for I am faint; therefore was his name called Edom.",
        }),

        ("genesis", 25, 31) => Some(Verse {
            content: "And Iacob said, Sell me this day thy birthright.",
        }),

        ("genesis", 25, 32) => Some(Verse {
            content: "And Esau said, Behold, I am at the point to die: and what profit shall this birthright doe to me?",
        }),

        ("genesis", 25, 33) => Some(Verse {
            content: "And Iacob said, Sweare to mee this day: and he sware to him: and he sold his birthright vnto Iacob.",
        }),

        ("genesis", 25, 34) => Some(Verse {
            content: "Then Iacob gaue Esau bread and pottage of lentiles; and he did eate and drinke, and rose up, and went his way: thus Esau despised his birthright.",
        }),

        ("genesis", 26, 1) => Some(Verse {
            content: "And there was a famine in the land, besides the first famine that was in the dayes of Abraham. And Isaac went vnto Abimelech King of the Philistims, vnto Gerar.",
        }),

        ("genesis", 26, 2) => Some(Verse {
            content: "And the LORD appeared vnto him and said, Goe not downe into Egypt; dwell in the land which I shall tell thee of.",
        }),

        ("genesis", 26, 3) => Some(Verse {
            content: "Soiourne in this land, and I wil be with thee, and will blesse thee: for vnto thee, and vnto thy seed I will giue all these countreys, and I wil performe the othe, which I sware vnto Abraham thy father.",
        }),

        ("genesis", 26, 4) => Some(Verse {
            content: "And I wil make thy seed to multiply as the starres of heauen, and will giue vnto thy seed all these countreys: and in thy Seed shall all the nations of the earth be blessed:",
        }),

        ("genesis", 26, 5) => Some(Verse {
            content: "Because that Abraham obeyed my voyce, and kept my charge, my Commandements, my Statutes and my Lawes.",
        }),

        ("genesis", 26, 6) => Some(Verse {
            content: "And Isaac dwelt in Gerar.",
        }),

        ("genesis", 26, 7) => Some(Verse {
            content: "And the men of the place asked him of his wife: and he said, She is my sister: for he feared to say, She is my wife; lest, said he, the men of the place should kill me for Rebekah, because shee was faire to looke vpon.",
        }),

        ("genesis", 26, 8) => Some(Verse {
            content: "And it came to passe when he had bene there a long time, that Abimelech king of the Philistims looked out at a window, and saw, and behold, Isaac was sporting with Rebekah his wife.",
        }),

        ("genesis", 26, 9) => Some(Verse {
            content: "And Abimelech called Isaac and said, Behold, of a suretie she is thy wife: and how saidst thou, She is my sister? And Isaac said vnto him, Because I said, Lest I die for her.",
        }),

        ("genesis", 26, 10) => Some(Verse {
            content: "And Abimelech said, What is this thou hast done vnto vs? one of the people might lightly haue lien with thy wife, and thou shouldest haue brought guiltinesse vpon vs.",
        }),

        ("genesis", 26, 11) => Some(Verse {
            content: "And Abimelech charged all his people, saying, Hee that toucheth this man or his wife, shall surely bee put to death.",
        }),

        ("genesis", 26, 12) => Some(Verse {
            content: "Then Isaac sowed in that land, and receiued in the same yeere an hundred fold: & the LORD blessed him.",
        }),

        ("genesis", 26, 13) => Some(Verse {
            content: "And the man waxed great, and went forward, and grew vntill he became very great.",
        }),

        ("genesis", 26, 14) => Some(Verse {
            content: "For he had possession of flocks, and possession of heards, and great store of seruants, and the Philistims enuied him.",
        }),

        ("genesis", 26, 15) => Some(Verse {
            content: "For all the wels which his fathers seruants had digged in the dayes of Abraham his father, the Philistims had stopped them, & filled them with earth.",
        }),

        ("genesis", 26, 16) => Some(Verse {
            content: "And Abimelech said vnto Isaac, Goe from vs: for thou art much mightier then we.",
        }),

        ("genesis", 26, 17) => Some(Verse {
            content: "And Isaac departed thence, and pitched his tent in the valley of Gerar, and dwelt there.",
        }),

        ("genesis", 26, 18) => Some(Verse {
            content: "And Isaac digged againe the wels of water, which they had digged in the dayes of Abraham his father: for the Philistims had stopped them after the death of Abraham, and he called their names after the names by which his father had called them.",
        }),

        ("genesis", 26, 19) => Some(Verse {
            content: "And Isaacs seruants digged in the valley, and found there a well of springing water.",
        }),

        ("genesis", 26, 20) => Some(Verse {
            content: "And the heardmen of Gerar did striue with Isaacs heardmen, saying, The water is ours; and hee called the name of the well, Esek, because they stroue with him.",
        }),

        ("genesis", 26, 21) => Some(Verse {
            content: "And they digged another well, and stroue for that also: and hee called the name of it, Sitnah.",
        }),

        ("genesis", 26, 22) => Some(Verse {
            content: "And he remoued from thence, and digged another well, and for that they stroue not: and he called the name of it Rehoboth: and he said, For now the LORD hath made roome for vs, and we shall be fruitfull in the land.",
        }),

        ("genesis", 26, 23) => Some(Verse {
            content: "And he went vp from thence to Beer-sheba.",
        }),

        ("genesis", 26, 24) => Some(Verse {
            content: "And the LORD appeared vnto him the same night, and saide, I am the God of Abraham thy father: feare not, for I am with thee, and will blesse thee, and multiply thy seede, for my seruant Abrahams sake.",
        }),

        ("genesis", 26, 25) => Some(Verse {
            content: "And he builded an altar there, and called vpon the name of the LORD, and pitched his tent there: and there Isaacs seruants digged a well.",
        }),

        ("genesis", 26, 26) => Some(Verse {
            content: "Then Abimelech went to him from Gerar, and Ahuzzath one of his friends, and Phichol the chiefe captaine of his armie.",
        }),

        ("genesis", 26, 27) => Some(Verse {
            content: "And Isaac saide vnto them, Wherefore come ye to me, seeing ye hate me, and haue sent me away from you?",
        }),

        ("genesis", 26, 28) => Some(Verse {
            content: "And they said, we saw certainly that the LORD was with thee: and wee said, Let there be now an othe betwixt vs, euen betwixt vs and thee, and let vs make a couenant with thee,",
        }),

        ("genesis", 26, 29) => Some(Verse {
            content: "That thou wilt doe vs no hurt, as we haue not touched thee, and as we haue done vnto thee nothing but good, and haue sent thee away in peace: thou art now the blessed of the LORD.",
        }),

        ("genesis", 26, 30) => Some(Verse {
            content: "And he made them a feast, and they did eate and drinke.",
        }),

        ("genesis", 26, 31) => Some(Verse {
            content: "And they rose vp betimes in the morning, and sware one to another: and Isaac sent them away, and they departed from him in peace.",
        }),

        ("genesis", 26, 32) => Some(Verse {
            content: "And it came to passe the same day, that Isaacs seruants came, and tolde him concerning the well which they had digged, and said vnto him, we haue found water.",
        }),

        ("genesis", 26, 33) => Some(Verse {
            content: "And he called it Shebah: therefore the name of the citie is Beer-sheba vnto this day.",
        }),

        ("genesis", 26, 34) => Some(Verse {
            content: "And Esau was forty yeeres old, when he tooke to wife Iudith, the daughter of Beeri the Hittite, and Bashemath the daughter of Elon the Hittite:",
        }),

        ("genesis", 26, 35) => Some(Verse {
            content: "Which were a griefe of minde vnto Isaac and to Rebekah.",
        }),

        ("genesis", 27, 1) => Some(Verse {
            content: "And it came to passe that when Isaac was old, and his eyes were dimme, so that he could not see, hee called Esau his eldest son, and said vnto him, My sonne. And hee said vnto him, Behold, here am I.",
        }),

        ("genesis", 27, 2) => Some(Verse {
            content: "And he said, Behold now, I am old, I know not the day of my death.",
        }),

        ("genesis", 27, 3) => Some(Verse {
            content: "Now therefore take, I pray thee, thy weapons, thy quiuer, and thy bow, and goe out to the field, and take mee some venison.",
        }),

        ("genesis", 27, 4) => Some(Verse {
            content: "And make me sauoury meat, such as I loue, and bring it to mee, that I may eate, that my soule may blesse thee before I die.",
        }),

        ("genesis", 27, 5) => Some(Verse {
            content: "And Rebekah heard when Isaac spake to Esau his sonne: and Esau went to the fielde to hunt for venison, and to bring it.",
        }),

        ("genesis", 27, 6) => Some(Verse {
            content: "And Rebekah spake vnto Iacob her sonne, saying, Behold, I heard thy father speake vnto Esau thy brother, saying,",
        }),

        ("genesis", 27, 7) => Some(Verse {
            content: "Bring me venison, and make mee sauoury meat, that I may eate, and blesse thee before the LORD, before my death.",
        }),

        ("genesis", 27, 8) => Some(Verse {
            content: "Now therefore, my sonne, obey my voyce, according to that which I command thee.",
        }),

        ("genesis", 27, 9) => Some(Verse {
            content: "Goe now to the flocke, and fetch me from thence two good kids of the goates, and I will make them sauoury meat for thy father, such as he loueth.",
        }),

        ("genesis", 27, 10) => Some(Verse {
            content: "And thou shalt bring it to thy father, that he may eate, and that he may blesse thee, before his death.",
        }),

        ("genesis", 27, 11) => Some(Verse {
            content: "And Iacob said to Rebekah his mother, Behold, Esau my brother is a hairy man, and I am a smooth man.",
        }),

        ("genesis", 27, 12) => Some(Verse {
            content: "My father peraduenture will feele me, and I shall seeme to him as a deceiuer, and I shall bring a curse vpon me, and not a blessing.",
        }),

        ("genesis", 27, 13) => Some(Verse {
            content: "And his mother said vnto him, Upon me be thy curse, my sonne: onely obey my voice, and go fetch me them.",
        }),

        ("genesis", 27, 14) => Some(Verse {
            content: "And hee went, and fetched, and brought them to his mother, and his mother made sauoury meat, such as his father loued.",
        }),

        ("genesis", 27, 15) => Some(Verse {
            content: "And Rebekah tooke goodly raiment of her eldest sonne Esau, which were with her in the house, and put them vpon Iacob her yonger sonne:",
        }),

        ("genesis", 27, 16) => Some(Verse {
            content: "And shee put the skinnes of the kids of the goats vpon his hands, and vpon the smooth of his necke.",
        }),

        ("genesis", 27, 17) => Some(Verse {
            content: "And she gaue the sauoury meate, and the bread, which she had prepared, into the hand of her sonne Iacob.",
        }),

        ("genesis", 27, 18) => Some(Verse {
            content: "And he came vnto his father, and said, My father: And he said, Here am I: who art thou, my sonne?",
        }),

        ("genesis", 27, 19) => Some(Verse {
            content: "And Iacob said vnto his father, I am Esau, thy first borne; I haue done according as thou badest mee: arise, I pray thee, sit, and eate of my venison, that thy soule may blesse me.",
        }),

        ("genesis", 27, 20) => Some(Verse {
            content: "And Isaac said vnto his sonne, How is it that thou hast found it so quickly, my sonne? And he said, Because the LORD thy God brought it to me.",
        }),

        ("genesis", 27, 21) => Some(Verse {
            content: "And Isaac saide vnto Iacob, Come neere, I pray thee, that I may feele thee, my sonne, whether thou bee my very sonne Esau, or not.",
        }),

        ("genesis", 27, 22) => Some(Verse {
            content: "And Iacob went neere vnto Isaac his father: and hee felt him, and said, The voyce is Iacobs voyce, but the hands are the hands of Esau.",
        }),

        ("genesis", 27, 23) => Some(Verse {
            content: "And he discerned him not, because his hands were hairie, as his brother Esaus hands: So he blessed him.",
        }),

        ("genesis", 27, 24) => Some(Verse {
            content: "And he said, Art thou my very sonne Esau? and he said, I am.",
        }),

        ("genesis", 27, 25) => Some(Verse {
            content: "And he said, Bring it neere to me, and I will eate of my sonnes venison, that my soule may blesse thee: and hee brought it neere to him, and he did eate: and he brought him wine, & he dranke.",
        }),

        ("genesis", 27, 26) => Some(Verse {
            content: "And his father Isaac saide vnto him, Come neere now, and kisse me, my sonne.",
        }),

        ("genesis", 27, 27) => Some(Verse {
            content: "And hee came neere, and kissed him: and he smelled the smell of his raiment, and blessed him, and said, See, the smell of my sonne is as the smell of a field, which the LORD hath blessed.",
        }),

        ("genesis", 27, 28) => Some(Verse {
            content: "Therefore God giue thee of the dew of heauen, and the fatnesse of the earth, and plenty of corne and wine.",
        }),

        ("genesis", 27, 29) => Some(Verse {
            content: "Let people serue thee, and nations bow downe to thee: bee lord ouer thy brethren, & let thy mothers sonnes bow downe to thee: Cursed bee euery one that curseth thee, and blessed be hee that blesseth thee.",
        }),

        ("genesis", 27, 30) => Some(Verse {
            content: "And it came to passe, as soone as Isaac had made an ende of blessing Iacob, and Iacob was yet scarce gone out from the presence of Isaac his father, that Esau his brother came in from his hunting.",
        }),

        ("genesis", 27, 31) => Some(Verse {
            content: "And hee also had made sauoury meate, and brought it vnto his father, and said vnto his father, Let my father arise, and eat of his sonnes venison, that thy soule may blesse me.",
        }),

        ("genesis", 27, 32) => Some(Verse {
            content: "And Isaac his father said vnto him, Who art thou? and he said, I am thy sonne, thy first borne Esau.",
        }),

        ("genesis", 27, 33) => Some(Verse {
            content: "And Isaac trembled very exceedingly, and said, Who? Where is he that hath taken venison, and brought it me, and I haue eaten of all before thou camest, and haue blessed him? yea and he shalbe blessed.",
        }),

        ("genesis", 27, 34) => Some(Verse {
            content: "And when Esau heard the words of his father, he cried with a great and exceeding bitter cry, and said vnto his father, Blesse mee, euen me also, O my father.",
        }),

        ("genesis", 27, 35) => Some(Verse {
            content: "And hee said, Thy brother came with subtilty, and hath taken away thy blessing.",
        }),

        ("genesis", 27, 36) => Some(Verse {
            content: "And he said, Is not he rightly naned Iacob? for he hath supplanted me these two times: hee tooke away my birthright, and behold, now he hath taken away my blessing: and hee said, Hast thou not reserued a blessing for mee?",
        }),

        ("genesis", 27, 37) => Some(Verse {
            content: "And Isaac answered and saide vnto Esau, Behold, I haue made him thy lord, and all his brethren haue I giuen to him for seruants: and with corne and wine haue I susteined him: and what shall I doe now vnto thee, my sonne?",
        }),

        ("genesis", 27, 38) => Some(Verse {
            content: "And Esau said vnto his father, Hast thou but one blessing, my father? blesse mee, euen mee also, O my father. And Esau lift vp his voyce, and wept.",
        }),

        ("genesis", 27, 39) => Some(Verse {
            content: "And Isaac his father answered, and said vnto him, Behold, thy dwelling shall be the fatnesse of the earth, and of the dew of heauen from aboue.",
        }),

        ("genesis", 27, 40) => Some(Verse {
            content: "And by thy sword shalt thou liue, and shalt serue thy brother: and it shall come to passe when thou shalt haue the dominion, that thou shalt breake his yoke from off thy necke.",
        }),

        ("genesis", 27, 41) => Some(Verse {
            content: "And Esau hated Iacob, because of the blessing, wherewith his father blessed him: and Esau said in his heart, The dayes of mourning for my father are at hand; then will I slay my brother Iacob.",
        }),

        ("genesis", 27, 42) => Some(Verse {
            content: "And these words of Esau her elder sonne were told to Rebekah: And shee sent and called Iacob her yonger sonne, and said vnto him, Behold, thy brother Esau, as touching thee, doeth comfort himselfe, purposing to kill thee.",
        }),

        ("genesis", 27, 43) => Some(Verse {
            content: "Now therefore my sonne, obey my voice: and arise, flee thou to Laban my brother, to Haran.",
        }),

        ("genesis", 27, 44) => Some(Verse {
            content: "And tary with him a few dayes, vntill thy brothers furie turne away;",
        }),

        ("genesis", 27, 45) => Some(Verse {
            content: "Untill thy brothers anger turne away from thee, and hee forget that, which thou hast done to him: then I will send, and fetch thee from thence: why should I be depriued also of you both in one day?",
        }),

        ("genesis", 27, 46) => Some(Verse {
            content: "And Rebekah said to Isaac, I am weary of my life, because of the daughters of Heth: If Iacob take a wife of the daughters of Heth, such as these which are of the daughters of the land, what good shall my life doe me?",
        }),

        ("genesis", 28, 1) => Some(Verse {
            content: "And Isaac called Iacob, and blessed him, and charged him, and saide vnto him, Thou shalt not take a wife, of the daughters of Canaan.",
        }),

        ("genesis", 28, 2) => Some(Verse {
            content: "Arise, goe to Padan Aram, to the house of Bethuel thy mothers father, and take thee a wife from thence, of the daughters of Laban thy mothers brother.",
        }),

        ("genesis", 28, 3) => Some(Verse {
            content: "And God Almighty blesse thee, and make thee fruitfull, and multiply thee, that thou mayest be a multitude of people:",
        }),

        ("genesis", 28, 4) => Some(Verse {
            content: "And giue thee the blessing of Abraham, to thee and to thy seede with thee, that thou mayest inherit the lande wherein thou art a stranger, which God gaue vnto Abraham.",
        }),

        ("genesis", 28, 5) => Some(Verse {
            content: "And Isaac sent away Iacob, and hee went to Padan-Aram vnto Laban, sonne of Bethuel the Syrian, the brother of Rebekah, Iacobs and Esaus mother.",
        }),

        ("genesis", 28, 6) => Some(Verse {
            content: "When Esau sawe that Isaac had blessed Iacob, and sent him away to Padan-Aram, to take him a wife from thence; and that as he blessed him, he gaue him a charge, saying, Thou shalt not take a wife of the daughters of Canaan;",
        }),

        ("genesis", 28, 7) => Some(Verse {
            content: "And that Iacob obeyed his father, and his mother, and was gone to Padan-Aram;",
        }),

        ("genesis", 28, 8) => Some(Verse {
            content: "And Esau seeing that the daughters of Canaan pleased not Isaac his father.",
        }),

        ("genesis", 28, 9) => Some(Verse {
            content: "Then went Esau vnto Ishmael, and tooke vnto the wiues which hee had, Mahalath the daughter of Ishmael Abrahams sonne, the sister of Nebaioth, to be his wife.",
        }),

        ("genesis", 28, 10) => Some(Verse {
            content: "And Iacob went out from Beer-sheba, and went toward Haran.",
        }),

        ("genesis", 28, 11) => Some(Verse {
            content: "And hee lighted vpon a certaine place, and taried there all night, because the sunne was set: and hee tooke of the stones of that place, and put them for his pillowes, and lay downe in that place to sleepe.",
        }),

        ("genesis", 28, 12) => Some(Verse {
            content: "And he dreamed, and beholde, a ladder set vp on the earth, and the top of it reached to heauen: and beholde the Angels of God ascending and descending on it.",
        }),

        ("genesis", 28, 13) => Some(Verse {
            content: "And behold, the LORD stood aboue it, and said, I am the LORD God of Abraham thy father, and the God of Isaac: the land whereon thou liest, to thee will I giue it, and to thy seede.",
        }),

        ("genesis", 28, 14) => Some(Verse {
            content: "And thy seed shall be as the dust of the earth, and thou shalt spread abroad to the West, and to the East, and to the North, and to the South: and in thee, and in thy seed, shall all the families of the earth be blessed.",
        }),

        ("genesis", 28, 15) => Some(Verse {
            content: "And behold, I am with thee, and will keepe thee in all places whither thou goest, and will bring thee againe into this land: for I will not leaue thee, vntill I haue done that which I haue spoken to thee of.",
        }),

        ("genesis", 28, 16) => Some(Verse {
            content: "And Iacob awaked out of his sleepe, and he said, Surely the LORD is in this place, and I knew it not.",
        }),

        ("genesis", 28, 17) => Some(Verse {
            content: "And he was afraid, and said, How dreadful is this place? this is none other, but the house of God, and this is the gate of heauen.",
        }),

        ("genesis", 28, 18) => Some(Verse {
            content: "And Iacob rose vp earely in the morning, and tooke the stone that hee had put for his pillowes, and set it vp for a pillar, and powred oile vpon the top of it.",
        }),

        ("genesis", 28, 19) => Some(Verse {
            content: "And hee called the name of that place Beth-el: but the name of that citie was called Luz, at the first.",
        }),

        ("genesis", 28, 20) => Some(Verse {
            content: "And Iacob vowed a vow, saying, If God will be with me, and will keepe me in this way that I goe, and will giue me bread to eate, and raiment to put on,",
        }),

        ("genesis", 28, 21) => Some(Verse {
            content: "So that I come againe to my fathers house in peace: then shall the LORD be my God.",
        }),

        ("genesis", 28, 22) => Some(Verse {
            content: "And this stone which I haue set for a pillar, shall be Gods house: and of all that thou shalt giue me, I will surely giue the tenth vnto thee.",
        }),

        ("genesis", 29, 1) => Some(Verse {
            content: "Then Iacob went on his iourney, and came into the land of the people of the East.",
        }),

        ("genesis", 29, 2) => Some(Verse {
            content: "And he looked, and behold, a well in the field, and loe, there were three flocks of sheepe lying by it: for out of that wel they watered the flocks: and a great stone was vpon the welles mouth.",
        }),

        ("genesis", 29, 3) => Some(Verse {
            content: "And thither were all the flockes gathered, and they rolled the stone from the wels mouth, & watered the sheepe, and put the stone againe vpon the wels mouth in his place.",
        }),

        ("genesis", 29, 4) => Some(Verse {
            content: "And Iacob said vnto them, My brethren, whence be ye? and they saide, Of Haran are we.",
        }),

        ("genesis", 29, 5) => Some(Verse {
            content: "And he said vnto them, Know ye Laban the sonne of Nahor? And they sayde, We knowe him.",
        }),

        ("genesis", 29, 6) => Some(Verse {
            content: "And he said vnto them, Is hee well? and they said, He is well: and behold, Rachel his daughter commeth with the sheepe.",
        }),

        ("genesis", 29, 7) => Some(Verse {
            content: "And hee said, Loe, it is yet high day, neither is it time that the cattell should be gathered together: water yee the sheepe, and goe and feed them.And hee said, Loe, it is yet high day, neither is it time that the cattell should be gathered together: water yee the sheepe, and goe and feed them.",
        }),

        ("genesis", 29, 8) => Some(Verse {
            content: "And they said, We cannot, vntill all the flockes bee gathered together, and till they rolle the stone from the welles mouth: then wee water the sheepe.",
        }),

        ("genesis", 29, 9) => Some(Verse {
            content: "And while hee yet spake with them, Rachel came with her fathers sheepe: for she kept them.",
        }),

        ("genesis", 29, 10) => Some(Verse {
            content: "And it came to passe, when Iacob saw Rachel the daughter of Laban his mothers brother, and the sheepe of Laban his mothers brother; that Iacob went neere, and rolled the stone from the wels mouth, and watered the flocke of Laban his mothers brother.",
        }),

        ("genesis", 29, 11) => Some(Verse {
            content: "And Iacob kissed Rachel, and lifted vp his voyce, and wept.",
        }),

        ("genesis", 29, 12) => Some(Verse {
            content: "And Iacob told Rachel, that hee was her fathers brother, and that hee was Rebekahs sonne: and she ranne, and told her father.",
        }),

        ("genesis", 29, 13) => Some(Verse {
            content: "And it came to passe, when Laban heard the tidings of Iacob his sisters sonne, that he ranne to meete him, and imbraced him, and kissed him, & brought him to his house: and hee tolde Laban all these things.",
        }),

        ("genesis", 29, 14) => Some(Verse {
            content: "And Laban said to him, Surely thou art my bone and my flesh: and he abode with him the space of a moneth.",
        }),

        ("genesis", 29, 15) => Some(Verse {
            content: "And Laban said vnto Iacob, Because thou art my brother, shouldest thou therefore serue me for nought? tell me, what shall thy wages be?",
        }),

        ("genesis", 29, 16) => Some(Verse {
            content: "And Laban had two daughters: the name of the elder was Leah, and the name of the yonger was Rachel.",
        }),

        ("genesis", 29, 17) => Some(Verse {
            content: "Leah was tender eyed: but Rachel was beautiful and well fauoured.",
        }),

        ("genesis", 29, 18) => Some(Verse {
            content: "And Iacob loued Rachel, and said, I will serue thee seuen yeeres for Rachel thy yonger daughter.",
        }),

        ("genesis", 29, 19) => Some(Verse {
            content: "And Laban said, It is better that I giue her to thee, then that I should giue her to another man: abide with mee.",
        }),

        ("genesis", 29, 20) => Some(Verse {
            content: "And Iacob serued seuen yeeres for Rachel: and they seemed vnto him but a few dayes, for the loue hee had to her.",
        }),

        ("genesis", 29, 21) => Some(Verse {
            content: "And Iacob said vnto Laban, Giue me my wife (for my dayes are fulfilled) that I may goe in vnto her.",
        }),

        ("genesis", 29, 22) => Some(Verse {
            content: "And Laban gathered together all the men of the place, and made a feast.",
        }),

        ("genesis", 29, 23) => Some(Verse {
            content: "And it came to passe in the euening, that he tooke Leah his daughter, and brought her to him, and he went in vnto her.",
        }),

        ("genesis", 29, 24) => Some(Verse {
            content: "And Laban gaue vnto his daughter Leah, Zilpah his mayde, for a handmayd.",
        }),

        ("genesis", 29, 25) => Some(Verse {
            content: "And it came to passe, that in the morning, behold it was Leah: and he said to Laban, What is this thou hast done vnto mee? did not I serue with thee for Rachel? wherefore then hast thou beguiled me?",
        }),

        ("genesis", 29, 26) => Some(Verse {
            content: "And Laban said, It must not be so done in our countrey, to giue the yonger, before the first borne.",
        }),

        ("genesis", 29, 27) => Some(Verse {
            content: "Fulfill her weeke, and wee will giue thee this also, for the seruice which thou shalt serue with mee, yet seuen other yeeres.",
        }),

        ("genesis", 29, 28) => Some(Verse {
            content: "And Iacob did so, and fulfilled her weeke: and he gaue him Rachel his daughter to wife also.",
        }),

        ("genesis", 29, 29) => Some(Verse {
            content: "And Laban gaue to Rachel his daughter, Bilhah his handmayd, to be her mayd.",
        }),

        ("genesis", 29, 30) => Some(Verse {
            content: "And hee went in also vnto Rachel, and he loued also Rachel more then Leah, and serued with him yet seuen other yeeres.",
        }),

        ("genesis", 29, 31) => Some(Verse {
            content: "And when the LORD saw that Leah was hated, hee opened her wombe: but Rachel was barren.",
        }),

        ("genesis", 29, 32) => Some(Verse {
            content: "And Leah conceiued and bare a sonne, and shee called his name Reuben: for she said, Surely, the LORD hath looked vpon my affliction; now therefore my husband will loue me.",
        }),

        ("genesis", 29, 33) => Some(Verse {
            content: "And shee conceiued againe, and bare a sonne, and saide, Because the LORD hath heard that I was hated, hee hath therefore giuen mee this sonne also, and she called his name Simeon.",
        }),

        ("genesis", 29, 34) => Some(Verse {
            content: "And shee conceiued againe, and bare a sonne, and said, Now this time will my husband be ioyned vnto me, because I haue borne him three sonnes: therefore was his name called Leui.",
        }),

        ("genesis", 29, 35) => Some(Verse {
            content: "And shee conceiued againe, and bare a sonne: and she said, Now wil I praise the LORD: therefore she called his name Iudah, and left bearing.",
        }),

        ("genesis", 30, 1) => Some(Verse {
            content: "And when Rachel saw that shee bare Iacob no children, Rachel enuied her sister, and said vnto Iacob, Giue mee children, or els I die.",
        }),

        ("genesis", 30, 2) => Some(Verse {
            content: "And Iacobs anger was kindled against Rachel, and he said, Am I in Gods stead, who hath withheld from thee the fruit of the wombe?",
        }),

        ("genesis", 30, 3) => Some(Verse {
            content: "And she said, Behold my mayde Bilhah: goe in vnto her, and she shall beare vpon my knees, that I may also have children by her.",
        }),

        ("genesis", 30, 4) => Some(Verse {
            content: "And shee gaue him Bilhah her handmayd to wife: and Iacob went in vnto her.",
        }),

        ("genesis", 30, 5) => Some(Verse {
            content: "And Bilhah conceiued and bare Iacob a sonne.",
        }),

        ("genesis", 30, 6) => Some(Verse {
            content: "And Rachel said, God hath iudged me, and hath also heard my voyce, and hath giuen me a sonne; therefore called she his name Dan.",
        }),

        ("genesis", 30, 7) => Some(Verse {
            content: "And Bilhah Rachels mayd conceiued againe, and bare Iacob a second sonne.",
        }),

        ("genesis", 30, 8) => Some(Verse {
            content: "And Rachel saide, With great wrastlings haue I wrastled with my sister, and I haue preuailed: and she called his name Naphtali.",
        }),

        ("genesis", 30, 9) => Some(Verse {
            content: "When Leah saw that she had left bearing, shee tooke Zilpah her mayde, and gaue her Iacob to wife.",
        }),

        ("genesis", 30, 10) => Some(Verse {
            content: "And Zilpah Leahs mayde bare Iacob a sonne.",
        }),

        ("genesis", 30, 11) => Some(Verse {
            content: "And Leah said, A troupe commeth: and she called his name Gad.",
        }),

        ("genesis", 30, 12) => Some(Verse {
            content: "And Zilpah Leahs mayde bare Iacob a second sonne.",
        }),

        ("genesis", 30, 13) => Some(Verse {
            content: "And Leah said, Happy am I, for the daughters will call me blessed: and she called his name Asher.",
        }),

        ("genesis", 30, 14) => Some(Verse {
            content: "And Reuben went in the dayes of wheat haruest, & found Mandrakes in the field, and brought them vnto his mother Leah. Then Rachel saide to Leah, Giue me, I pray thee, of thy sonnes Mandrakes.",
        }),

        ("genesis", 30, 15) => Some(Verse {
            content: "And shee said vnto her, Is it a small matter, that thou hast taken my husband? and wouldst thou take away my sonnes Mandrakes also? and Rachel said, Therefore hee shall lye with thee to night, for thy sonnes Mandrakes.",
        }),

        ("genesis", 30, 16) => Some(Verse {
            content: "And Iacob came out of the field in the euening, and Leah went out to meet him, and said, Thou must come in vnto mee: for surely I haue hired thee with my sonnes Mandrakes. And hee lay with her that night.",
        }),

        ("genesis", 30, 17) => Some(Verse {
            content: "And God hearkened vnto Leah, and she conceiued, and bare Iacob the fift sonne.",
        }),

        ("genesis", 30, 18) => Some(Verse {
            content: "And Leah said, God hath giuen mee my hire, because I haue giuen my mayden to my husband: and she called his name Issachar.",
        }),

        ("genesis", 30, 19) => Some(Verse {
            content: "And Leah conceiued againe, and bare Iacob the sixth sonne.",
        }),

        ("genesis", 30, 20) => Some(Verse {
            content: "And Leah said, God hath endued me with a good dowry: Now will my husband dwel with me, because I haue borne him sixe sonnes: and shee called his name Zebulun.",
        }),

        ("genesis", 30, 21) => Some(Verse {
            content: "And afterwardes shee bare a daughter, and called her name Dinah.",
        }),

        ("genesis", 30, 22) => Some(Verse {
            content: "And God remembred Rachel, and God hearkened to her, and opened her wombe.",
        }),

        ("genesis", 30, 23) => Some(Verse {
            content: "And shee conceiued and bare a sonne, and said; God hath taken away my reproch:",
        }),

        ("genesis", 30, 24) => Some(Verse {
            content: "And shee called his name Ioseph, and saide, The LORD shall adde to me another sonne.",
        }),

        ("genesis", 30, 25) => Some(Verse {
            content: "And it came to passe when Rachel had borne Ioseph, that Iacob said vnto Laban, Send me away, that I may goe vnto mine owne place, and to my countrey.",
        }),

        ("genesis", 30, 26) => Some(Verse {
            content: "Giue mee my wiues and my children, for whom I haue serued thee, and let me goe: for thou knowest my seruice which I haue done thee.",
        }),

        ("genesis", 30, 27) => Some(Verse {
            content: "And Laban said vnto him, I pray thee, if I haue found fauour in thine eyes, tary: for I haue learned by experience, that the LORD hath blessed me for thy sake.",
        }),

        ("genesis", 30, 28) => Some(Verse {
            content: "And he said, Appoint me thy wages, and I will giue it.",
        }),

        ("genesis", 30, 29) => Some(Verse {
            content: "And hee said vnto him, Thou knowest how I haue serued thee, and how thy cattell was with me.",
        }),

        ("genesis", 30, 30) => Some(Verse {
            content: "For it was little which thou hadst before I came; and it is now increased vnto a multitude; and the LORD hath blessed thee since my comming: and now when shall I prouide for mine owne house also?",
        }),

        ("genesis", 30, 31) => Some(Verse {
            content: "And hee said, what shall I giue thee? and Iacob said, Thou shalt not giue me any thing; if thou wilt doe this thing for mee, I will againe feed and keepe thy flocke.",
        }),

        ("genesis", 30, 32) => Some(Verse {
            content: "I will passe through all thy flocke to day, remoouing from thence all the speckled and spotted cattell: and all the browne cattell among the sheepe, and the spotted and speckled among the goates, and of such shalbe my hire.",
        }),

        ("genesis", 30, 33) => Some(Verse {
            content: "So shall my righteousnesse answere for mee in time to come, when it shall come for my hire, before thy face: euery one that is not speckled and spotted amongst the goates, and browne amongst the sheepe, that shalbe counted stollen with me.",
        }),

        ("genesis", 30, 34) => Some(Verse {
            content: "And Laban saide, Beholde, I would it might bee according to thy word.",
        }),

        ("genesis", 30, 35) => Some(Verse {
            content: "And he remoued that day the hee goates that were ring-straked, and spotted, and all the shee goats that were speckled and spotted, and euery one that had some white in it, and all the browne amongst the sheepe, and gaue them into the hand of his sonnes.",
        }),

        ("genesis", 30, 36) => Some(Verse {
            content: "And hee set three dayes iourney betwixt himselfe and Iacob: and Iacob fed the rest of Labans flocks.",
        }),

        ("genesis", 30, 37) => Some(Verse {
            content: "And Iacob tooke him rods of greene poplar, and of the hasel and chesnut tree, and pilled white strakes in them, and made the white appeare which was in the rods.",
        }),

        ("genesis", 30, 38) => Some(Verse {
            content: "And he set the rods which he had pilled, before the flockes in the gutters in the watering troughes when the flocks came to drinke, that they should conceiue when they came to drinke.",
        }),

        ("genesis", 30, 39) => Some(Verse {
            content: "And the flockes conceiued before the rods, and brought forth cattell ringstraked, speckled and spotted.",
        }),

        ("genesis", 30, 40) => Some(Verse {
            content: "And Iacob did separate the lambes, and set the faces of the flockes toward the ring-straked, and all the browne in the flocke of Laban: and he put his owne flocks by themselues, and put them not vnto Labans cattell.",
        }),

        ("genesis", 30, 41) => Some(Verse {
            content: "And it came to passe whensoeuer the stronger cattell did conceiue, that Iacob layd the rods before the eyes of the cattell in the gutters, that they might conceiue among the rods.",
        }),

        ("genesis", 30, 42) => Some(Verse {
            content: "But when the cattel were feeble, hee put them not in: so the feebler were Labans, and the stronger Iacobs.",
        }),

        ("genesis", 30, 43) => Some(Verse {
            content: "And the man increased exceedingly, and had much cattell, and maydseruants, and men seruants, and camels, and asses.",
        }),

        ("genesis", 31, 1) => Some(Verse {
            content: "And he heard the words of Labans sonnes, saying, Iacob hath taken away all that was our fathers; and of that which was of our fathers, hath hee gotten all this glory.",
        }),

        ("genesis", 31, 2) => Some(Verse {
            content: "And Iacob behelde the countenance of Laban, and behold, it was not toward him as before.",
        }),

        ("genesis", 31, 3) => Some(Verse {
            content: "And the LORD said vnto Iacob, Returne vnto the land of thy fathers, and to thy kindred; and I wil be with thee.",
        }),

        ("genesis", 31, 4) => Some(Verse {
            content: "And Iacob sent and called Rachel and Leah, to the field vnto his flocke,",
        }),

        ("genesis", 31, 5) => Some(Verse {
            content: "And said vnto them, I see your fathers countenance, that it is not toward mee as before: but the God of my father hath bene with me.",
        }),

        ("genesis", 31, 6) => Some(Verse {
            content: "And yee know, that with all my power I haue serued your father.",
        }),

        ("genesis", 31, 7) => Some(Verse {
            content: "And your father hath deceiued mee, and changed my wages ten times: but God suffered him not to hurt me.",
        }),

        ("genesis", 31, 8) => Some(Verse {
            content: "If hee said thus, The speckled shall be thy wages, then all the cattell bare speckled: and if he said thus, The ring-straked shalbe thy hire, then bare all the cattell ring-straked.",
        }),

        ("genesis", 31, 9) => Some(Verse {
            content: "Thus God hath taken away the cattell of your father, and giuen them to mee.",
        }),

        ("genesis", 31, 10) => Some(Verse {
            content: "And it came to passe at the time that the cattell conceiued, that I lifted vp mine eyes and saw in a dreame, and behold, the rammes which leaped vpon the cattell were ring-straked, speckled and grisled.",
        }),

        ("genesis", 31, 11) => Some(Verse {
            content: "And the Angel of God spake vnto me in a dreame, saying, Iacob; And I said, Here am I.",
        }),

        ("genesis", 31, 12) => Some(Verse {
            content: "And hee said, Lift vp now thine eyes, and see, all the rammes which leape vpon the cattell are ring-straked, speckled and grisled: for I haue seene all that Laban doeth vnto thee.",
        }),

        ("genesis", 31, 13) => Some(Verse {
            content: "I am the God of Bethel, where thou annoyntedst the pillar, and where thou vowedst a vow vnto mee: now arise, get thee out from this land, and returne vnto the land of thy kindred.",
        }),

        ("genesis", 31, 14) => Some(Verse {
            content: "And Rachel and Leah answered, and said vnto him; Is there yet any portion or inheritance for vs in our fathers house?",
        }),

        ("genesis", 31, 15) => Some(Verse {
            content: "Are we not counted of him strangers? for he hath sold vs, and hath quite deuoured also our money.",
        }),

        ("genesis", 31, 16) => Some(Verse {
            content: "For all the riches which God hath taken from our father, that is ours, and our childrens: now then whatsoeuer God hath said vnto thee, doe.",
        }),

        ("genesis", 31, 17) => Some(Verse {
            content: "Then Iacob rose vp, and set his sonnes and his wiues vpon camels.",
        }),

        ("genesis", 31, 18) => Some(Verse {
            content: "And he caried away all his cattell, and all his goods which he had gotten, the cattell of his getting, which hee had gotten in Padan Aram, for to goe to Isaac his father in the land of Canaan.",
        }),

        ("genesis", 31, 19) => Some(Verse {
            content: "And Laban went to sheare his sheepe: and Rachel had stollen the Images that were her fathers.",
        }),

        ("genesis", 31, 20) => Some(Verse {
            content: "And Iacob stale away vnawares to Laban the Syrian, in that he told him not that he fled.",
        }),

        ("genesis", 31, 21) => Some(Verse {
            content: "So hee fled with all that hee had, and he rose vp and passed ouer the Riuer, and set his face toward the mount Gilead.",
        }),

        ("genesis", 31, 22) => Some(Verse {
            content: "And it was tolde Laban on the third day, that Iacob was fled.",
        }),

        ("genesis", 31, 23) => Some(Verse {
            content: "And hee tooke his brethren with him, and pursued after him seuen dayes iourney, and they ouertooke him in the mount Gilead.",
        }),

        ("genesis", 31, 24) => Some(Verse {
            content: "And God came to Laban the Syrian in a dreame by night, and saide vnto him, Take heed that thou speake not to Iacob either good or bad.",
        }),

        ("genesis", 31, 25) => Some(Verse {
            content: "Then Laban ouertooke Iacob. Now Iacob had pitched his tent in the mount: and Laban with his brethren pitched in the mount of Gilead.",
        }),

        ("genesis", 31, 26) => Some(Verse {
            content: "And Laban said to Iacob, What hast thou done, that thou hast stollen away vnawares to me, and caried away my daughters, as captiues taken with the sword?",
        }),

        ("genesis", 31, 27) => Some(Verse {
            content: "Wherefore didst thou flie away secretly, and steale away from me, and didst not tell mee? that I might haue sent thee away with mirth, and with songs, with tabret, and with harpe,",
        }),

        ("genesis", 31, 28) => Some(Verse {
            content: "And hast not suffered me to kisse my sonnes and my daughters? thou hast now done foolishly in so doing.",
        }),

        ("genesis", 31, 29) => Some(Verse {
            content: "It is in the power of my hand to doe you hurt: but the God of your father spake vnto mee yesternight, saying, Take thou heed, that thou speake not to Iacob either good or bad.",
        }),

        ("genesis", 31, 30) => Some(Verse {
            content: "And now though thou wouldest needes bee gone, because thou sore longedst after thy fathers house; yet wherefore hast thou stollen my gods?",
        }),

        ("genesis", 31, 31) => Some(Verse {
            content: "And Iacob answered and said to Laban, Because I was afraid: for I said, Peraduenture thou wouldest take by force thy daughters from me.",
        }),

        ("genesis", 31, 32) => Some(Verse {
            content: "With whomsoeuer thou findest thy gods, let him not liue: before our brethren discerne thou what is thine with me, and take it to thee: for Iacob knew not that Rachel had stollen them.",
        }),

        ("genesis", 31, 33) => Some(Verse {
            content: "And Laban went into Iacobs tent, and into Leahs tent, and into the two maid seruants tents: but he found them not. Then went he out of Leahs tent, and entred into Rachels tent.",
        }),

        ("genesis", 31, 34) => Some(Verse {
            content: "Now Rachel had taken the images, and put them in the camels furniture, and sate vpon them: and Laban searched all the tent, but found them not.",
        }),

        ("genesis", 31, 35) => Some(Verse {
            content: "And shee said to her father, Let it not displease my lord, that I cannot rise vp before thee; for the custome of women is vpon mee: and he searched, but found not the images.",
        }),

        ("genesis", 31, 36) => Some(Verse {
            content: "And Iacob was wroth, and chode with Laban: and Iacob answered and said to Laban, what is my trespasse? what is my sinne, that thou hast so hotly pursued after me?",
        }),

        ("genesis", 31, 37) => Some(Verse {
            content: "Whereas thou hast searched all my stuffe, what hast thou found of all thy houshold stuffe? set it here before my brethren, and thy brethren, that they may iudge betwixt vs both.",
        }),

        ("genesis", 31, 38) => Some(Verse {
            content: "This twentie yeeres haue I bene with thee: thy ewes and thy shee goates haue not cast their yong, and the rammes of thy flocke haue I not eaten.",
        }),

        ("genesis", 31, 39) => Some(Verse {
            content: "That which was torne of beasts, I brought not vnto thee: I bare the losse of it; of my hand didst thou require it, whether stollen by day, or stollen by night.",
        }),

        ("genesis", 31, 40) => Some(Verse {
            content: "Thus I was in þe day, the drought consumed mee, and the frost by night, aud my sleep departed from mine eyes.",
        }),

        ("genesis", 31, 41) => Some(Verse {
            content: "Thus have I bene twentie yeres in thy house: I serued thee fourteene yeeres for thy two daughters, and sixe yeres for thy cattel; and thou hast changed my wages ten times.",
        }),

        ("genesis", 31, 42) => Some(Verse {
            content: "Except the God of my father, the God of Abraham, and the feare of Isaac had bin with me, surely thou hadst sent me away now emptie: God hath seene mine affliction, and the labour of my hands, & rebuked thee yesternight.",
        }),

        ("genesis", 31, 43) => Some(Verse {
            content: "And Laban answered and said vnto Iacob, These daughters are my daughters, and these children are my children, and these cattell are my cattell, and all that thou seest, is mine: and what can I doe this day vnto these my daughters, or vnto their children which they haue borne?",
        }),

        ("genesis", 31, 44) => Some(Verse {
            content: "Now therefore come thou, let vs make a couenant, I and thou: and let it be for a witnesse betweene me and thee.",
        }),

        ("genesis", 31, 45) => Some(Verse {
            content: "And Iacob tooke a stone, and set it vp for a pillar.",
        }),

        ("genesis", 31, 46) => Some(Verse {
            content: "And Iacob saide vnto his brethren, Gather stones: and they tooke stones, and made an heape, and they did eate there vpon the heape.",
        }),

        ("genesis", 31, 47) => Some(Verse {
            content: "And Laban called it Iegar-Sahadutha: but Iacob called it Galeed.",
        }),

        ("genesis", 31, 48) => Some(Verse {
            content: "And Laban said, This heape is a witnesse betweene mee and thee this day. Therefore was the name of it called Galeed,",
        }),

        ("genesis", 31, 49) => Some(Verse {
            content: "And Mizpah: for he said, The LORD watch betweene me and thee when we are absent one from another.",
        }),

        ("genesis", 31, 50) => Some(Verse {
            content: "If thou shalt afflict my daughters, or if thou shalt take other wiues beside my daughters, no man is with vs; See, God is witnesse betwixt mee and thee.",
        }),

        ("genesis", 31, 51) => Some(Verse {
            content: "And Laban said to Iacob, Behold this heape, and behold this pillar, which I haue cast betwixt me and thee.",
        }),

        ("genesis", 31, 52) => Some(Verse {
            content: "This heape be witnesse, and this pillar be witnesse, that I will not passe ouer this heape to thee, and that thou shalt not passe ouer this heape, and this pillar vnto me, for harme.",
        }),

        ("genesis", 31, 53) => Some(Verse {
            content: "The God of Abraham, and the God of Nahor, the God of their father, iudge betwixt vs. And Iacob sware by the feare of his father Isaac.",
        }),

        ("genesis", 31, 54) => Some(Verse {
            content: "Then Iacob offred sacrifice vpon the mount, and called his brethren to eate bread, and they did eate bread, and taried all night in the mount.",
        }),

        ("genesis", 31, 55) => Some(Verse {
            content: "And earely in the morning, Laban rose vp and kissed his sonnes, and his daughters, and blessed them: and Laban departed, and returned vnto his place.",
        }),

        ("genesis", 32, 1) => Some(Verse {
            content: "And Iacob went on his way, and the Angels of God met him.",
        }),

        ("genesis", 32, 2) => Some(Verse {
            content: "And when Iacob saw them, he said, This is Gods hoste: and hee called the name of that place Mahanaim.",
        }),

        ("genesis", 32, 3) => Some(Verse {
            content: "And Iacob sent messengers before him, to Esau his brother, vnto the land of Seir, the countrey of Edom.",
        }),

        ("genesis", 32, 4) => Some(Verse {
            content: "And he commaunded them, saying, Thus shall ye speake vnto my lord Esau, Thy seruant Iacob saith thus, I haue soiourned with Laban, and stayed there vntill now.",
        }),

        ("genesis", 32, 5) => Some(Verse {
            content: "And I haue oxen, and asses, flockes, and men seruants and women seruants: and I haue sent to tell my lord, that I may find grace in thy sight.",
        }),

        ("genesis", 32, 6) => Some(Verse {
            content: "And the messengers returned to Iacob, saying, Wee came to thy brother Esau, and also he commeth to meet thee, and foure hundred men with him.",
        }),

        ("genesis", 32, 7) => Some(Verse {
            content: "Then Iacob was greatly afraid, and distressed, and he diuided the people that was with him, and the flockes, and herdes, and the camels into two bands,",
        }),

        ("genesis", 32, 8) => Some(Verse {
            content: "And said, If Esau come to the one company, and smite it, then the other company which is left, shall escape.",
        }),

        ("genesis", 32, 9) => Some(Verse {
            content: "And Iacob said, O God of my father Abraham, and God of my father Isaac, the LORD which saidst vnto me, Returne vnto thy countrey, and to thy kinred, and I will deale well with thee:",
        }),

        ("genesis", 32, 10) => Some(Verse {
            content: "I am not worthy of the least of all the mercies, and of all the trueth, which thou hast shewed vnto thy seruant: for with my staffe I passed ouer this Iordan, and now I am become two bands.",
        }),

        ("genesis", 32, 11) => Some(Verse {
            content: "Deliuer me, I pray thee, from the hand of my brother, from the hand of Esau: for I feare him, lest he will come, and smite me, and the mother with the children.",
        }),

        ("genesis", 32, 12) => Some(Verse {
            content: "And thou saidst, I will surely doe thee good, and make thy seed as the sand of the sea, which cannot be numbred for multitude.",
        }),

        ("genesis", 32, 13) => Some(Verse {
            content: "And he lodged there that same night, and tooke of that which came to his hand, a present for Esau his brother:",
        }),

        ("genesis", 32, 14) => Some(Verse {
            content: "Two hundred shee goats, and twentie hee goats, two hundred ewes, and twentie rammes,",
        }),

        ("genesis", 32, 15) => Some(Verse {
            content: "Thirtie milch camels with their colts, fortie kine, and ten bulles, twenty shee ashes, and ten foales.",
        }),

        ("genesis", 32, 16) => Some(Verse {
            content: "And hee deliuered them into the hand of his seruants, euery droue by themselues, and said vnto his seruants, Passe ouer before me, and put a space betwixt droue and droue.",
        }),

        ("genesis", 32, 17) => Some(Verse {
            content: "And he commanded the formost, saying, When Esau my brother meeteth thee, and asketh thee, saying, whose art thou? and whither goest thou? and whose are these before thee?",
        }),

        ("genesis", 32, 18) => Some(Verse {
            content: "Then thou shalt say, They be thy seruant Iacobs: it is a present sent vnto my lord Esau: and behold also, he is behind vs.",
        }),

        ("genesis", 32, 19) => Some(Verse {
            content: "And so commanded he the second, and the third, and all that followed the droues, saying, On this maner shal you speake vnto Esau, when you find him.",
        }),

        ("genesis", 32, 20) => Some(Verse {
            content: "And say ye moreouer, Beholde, thy seruant Iacob is behind vs: for he said, I will appease him with the present that goeth before me, and afterward I will see his face; peraduenture he will accept of me.",
        }),

        ("genesis", 32, 21) => Some(Verse {
            content: "So went the present ouer before him: and himselfe lodged that night in the company.",
        }),

        ("genesis", 32, 22) => Some(Verse {
            content: "And hee rose vp that night, and tooke his two wiues, and his two women seruants, and his eleuen sonnes, and passed ouer the foord Iabbok.",
        }),

        ("genesis", 32, 23) => Some(Verse {
            content: "And he tooke them, and sent them ouer the brooke, and sent ouer that hee had.",
        }),

        ("genesis", 32, 24) => Some(Verse {
            content: "And Iacob was left alone: and there wrestled a man with him, vntill the breaking of the day.",
        }),

        ("genesis", 32, 25) => Some(Verse {
            content: "And when he saw, that he preuailed not against him, he touched the hollow of his thigh: and the hollow of Iacobs thigh was out of ioynt, as hee wrestled with him.",
        }),

        ("genesis", 32, 26) => Some(Verse {
            content: "And he said, Let me goe, for the day breaketh: and he said, I will not let thee goe, except thou blesse me.",
        }),

        ("genesis", 32, 27) => Some(Verse {
            content: "And he said vnto him, what is thy name? and he said, Iacob.",
        }),

        ("genesis", 32, 28) => Some(Verse {
            content: "And he said, Thy name shall be called no more Iacob, but Israel: for as a prince hast thou power with God, and with men, and hast preuailed.",
        }),

        ("genesis", 32, 29) => Some(Verse {
            content: "And Iacob asked him, and saide, Tell me, I pray thee, thy name: and he said, wherefore is it, that thou doest aske after my name? and he blessed him there.",
        }),

        ("genesis", 32, 30) => Some(Verse {
            content: "And Iacob called the name of the place Peniel: for I haue seene God face to face, and my life is preserued.",
        }),

        ("genesis", 32, 31) => Some(Verse {
            content: "And as he passed ouer Penuel, the sunne rose vpon him, and he halted vpon his thigh.",
        }),

        ("genesis", 32, 32) => Some(Verse {
            content: "Therefore the children of Israel eate not of the sinewe which shranke, which is vpon the hollow of the thigh, vnto this day: because hee touched the hollow of Iacobs thigh, in the sinewe that shranke.",
        }),

        ("genesis", 33, 1) => Some(Verse {
            content: "And Iacob lifted vp his eyes, and looked, and behold, Esau came, and with him foure hundreth men: and hee diuided the children vnto Leah, and vnto Rachel, and vnto the two handmaids.",
        }),

        ("genesis", 33, 2) => Some(Verse {
            content: "And he put the handmaides, and their chidren foremost, and Leah and her children after, and Rachel and Ioseph hindermost.",
        }),

        ("genesis", 33, 3) => Some(Verse {
            content: "And hee passed ouer before them, and bowed himselfe to the ground seuen times, vntill hee came neere to his brother.",
        }),

        ("genesis", 33, 4) => Some(Verse {
            content: "And Esau ran to meete him, and imbraced him, and fell on his necke, and kissed him, and they wept.",
        }),

        ("genesis", 33, 5) => Some(Verse {
            content: "And he lift vp his eyes, and sawe the women, and the children, and said, who are those with thee? And he said, The children which God hath graciously giuen thy seruant.",
        }),

        ("genesis", 33, 6) => Some(Verse {
            content: "Then the handmaidens came neere; they and their children, and they bowed themselues.",
        }),

        ("genesis", 33, 7) => Some(Verse {
            content: "And Leah also with her children came neere, and bowed themselues: and after came Ioseph neere and Rachel, and they bowed themselues.",
        }),

        ("genesis", 33, 8) => Some(Verse {
            content: "And he said, What meanest thou by all this droue, which I met? And he said, These are to find grace in the sight of my lord.",
        }),

        ("genesis", 33, 9) => Some(Verse {
            content: "And Esau said, I haue enough: my brother, keepe that thou hast vnto thy selfe.",
        }),

        ("genesis", 33, 10) => Some(Verse {
            content: "And Iacob saide, Nay, I pray thee: if now I haue found grace in thy sight, then receiue my present at my hand: for therefore I haue seene thy face, as though I had seene the face of God; and thou wast pleased with me.",
        }),

        ("genesis", 33, 11) => Some(Verse {
            content: "Take, I pray thee, my blessing that is brought to thee; because God hath dealt graciously with mee, and because I haue enough: and hee vrged him, and he tooke it.",
        }),

        ("genesis", 33, 12) => Some(Verse {
            content: "And he said, Let vs take our iourney, and let vs goe, and I will goe before thee.",
        }),

        ("genesis", 33, 13) => Some(Verse {
            content: "And hee said vnto him, My lord knoweth, that the children are tender, and the flockes and heards with yong are with mee: and if men should ouer-driue them one day, all the flocke will die.",
        }),

        ("genesis", 33, 14) => Some(Verse {
            content: "Let my lord, I pray thee, passe ouer before his seruant, and I will leade on softly, according as the cattell that goeth before me, and the children be able to endure, vntill I come vnto my lord vnto Seir.",
        }),

        ("genesis", 33, 15) => Some(Verse {
            content: "And Esau said, Let me now leaue with thee some of the folke that are with me: And hee said, What needeth it? let me finde grace in the sight of my lord.",
        }),

        ("genesis", 33, 16) => Some(Verse {
            content: "So Esau returned that day, on his way vnto Seir.",
        }),

        ("genesis", 33, 17) => Some(Verse {
            content: "And Iacob iourneyed to Succoth, and built him an house, and made boothes for his cattell: therefore the name of the place is called Succoth.",
        }),

        ("genesis", 33, 18) => Some(Verse {
            content: "And Iacob came to Shalem, a citie of Shechem, which is the land of Canaan, when he came from Padan Aram, and pitched his tent before the Citie.",
        }),

        ("genesis", 33, 19) => Some(Verse {
            content: "And he bought a parcell of a field where hee had spread his tent, at the hand of the children of Hamor Shechems father, for an hundred pieces of money.",
        }),

        ("genesis", 33, 20) => Some(Verse {
            content: "And hee erected there an Altar, and called it El-Elohe-Israel.",
        }),

        ("genesis", 34, 1) => Some(Verse {
            content: "And Dinah the daughter of Leah, which shee bare vnto Iacob, went out to see the daughters of the land.",
        }),

        ("genesis", 34, 2) => Some(Verse {
            content: "And when Shechem the sonne of Hamor the Hiuite, prince of the countrey saw her, he tooke her, and lay with her, and defiled her.",
        }),

        ("genesis", 34, 3) => Some(Verse {
            content: "And his soule claue vnto Dinah the daughter of Iacob, and hee loued the damsell, and spake kindly vnto the damsell.",
        }),

        ("genesis", 34, 4) => Some(Verse {
            content: "And Shechem spake vnto his father Hamor, saying, Get mee this damsell to wife.",
        }),

        ("genesis", 34, 5) => Some(Verse {
            content: "And Iacob heard that he had defiled Dinah his daughter (now his sonnes were with his cattel in the field) and Iacob helde his peace vntill they were come.",
        }),

        ("genesis", 34, 6) => Some(Verse {
            content: "And Hamor the father of Shechem went out vnto Iacob to commune with him.",
        }),

        ("genesis", 34, 7) => Some(Verse {
            content: "And the sonnes of Iacob came out of the field when they heard it, and the men were grieued: and they were very wroth, because hee had wrought folly in Israel, in lying with Iacobs daughter; which thing ought not to be done.",
        }),

        ("genesis", 34, 8) => Some(Verse {
            content: "And Hamor communed with them, saying, The soule of my sonne Shechem longeth for your daughter: I pray you giue her him to wife.",
        }),

        ("genesis", 34, 9) => Some(Verse {
            content: "And make ye mariages with vs, and giue your daughters vnto vs, and take our daughters vnto you.",
        }),

        ("genesis", 34, 10) => Some(Verse {
            content: "And ye shall dwell with vs, and the land shall be before you: dwell and trade you therein, and get you possessions therein.",
        }),

        ("genesis", 34, 11) => Some(Verse {
            content: "And Shechem said vnto her father, and vnto her brethren, Let mee finde grace in your eyes, and what yee shall say vnto me, I will giue.",
        }),

        ("genesis", 34, 12) => Some(Verse {
            content: "Aske mee neuer so much dowrie and gift, and I will giue according as yee shall say vnto mee: but giue me the damsell to wife.",
        }),

        ("genesis", 34, 13) => Some(Verse {
            content: "And the sonnes of Iacob answered Shechem, and Hamor his father deceitfully, and said, because he had defiled Dinah their sister.",
        }),

        ("genesis", 34, 14) => Some(Verse {
            content: "And they saide vnto them, wee cannot doe this thing, to giue our sister to one that is vncircumcised: for that were a reproch vnto vs.",
        }),

        ("genesis", 34, 15) => Some(Verse {
            content: "But in this will we consent vnto you: If ye will be as we be, that euery male of you be circumcised:",
        }),

        ("genesis", 34, 16) => Some(Verse {
            content: "Then wil we giue our daughters vnto you, and we wil take your daughters to vs, and we will dwell with you, and we will become one people.",
        }),

        ("genesis", 34, 17) => Some(Verse {
            content: "But if ye will not hearken vnto vs, to be circumcised, then will we take our daughter, and we will be gone.",
        }),

        ("genesis", 34, 18) => Some(Verse {
            content: "And their words pleased Hamor, and Shechem Hamors sonne.",
        }),

        ("genesis", 34, 19) => Some(Verse {
            content: "And the yong man deferred not to doe the thing, because he had delight in Iacobs daughter: and he was more honourable then all the house of his father.",
        }),

        ("genesis", 34, 20) => Some(Verse {
            content: "And Hamor and Shechem his sonne came vnto the gate of their citie, and communed with the men of their citie, saying:",
        }),

        ("genesis", 34, 21) => Some(Verse {
            content: "These men are peaceable with vs, therefore let them dwel in the land, and trade therein: for the land, behold, it is large enough for them: let vs take their daughters to vs for wiues, and let vs giue them our daughters.",
        }),

        ("genesis", 34, 22) => Some(Verse {
            content: "Onely herein will the men consent vnto vs, for to dwell with vs to be one people, if euery male among vs bee circumcised, as they are circumcised.",
        }),

        ("genesis", 34, 23) => Some(Verse {
            content: "Shall not their cattell, and their substance, and euery beast of theirs bee ours? onely let vs consent vnto them, and they will dwell with vs.",
        }),

        ("genesis", 34, 24) => Some(Verse {
            content: "And vnto Hamor and vnto Shechem his sonne, hearkened all that went out of the gate of his citie; and euery male was circumcised, all that went out of the gate of his citie.",
        }),

        ("genesis", 34, 25) => Some(Verse {
            content: "And it came to passe on the thirde day when they were sore, that two of the sonnes of Iacob, Simeon and Leui, Dinahs brethren, tooke each man his sword and came vpon the citie boldly, and slew all the males.",
        }),

        ("genesis", 34, 26) => Some(Verse {
            content: "And they slew Hamor and Shechem his sonne, with the edge of the sword, and tooke Dinah out of Shechems house, and went out.",
        }),

        ("genesis", 34, 27) => Some(Verse {
            content: "The sonnes of Iacob came vpon the slaine, and spoiled the citie, because they had defiled their sister.",
        }),

        ("genesis", 34, 28) => Some(Verse {
            content: "They tooke their sheepe, and their oxen, and their asses, and that which was in the citie, and that which was in the field.",
        }),

        ("genesis", 34, 29) => Some(Verse {
            content: "And all their wealth, and all their little ones, and their wiues tooke they captiue, and spoiled euen all that was in the house.",
        }),

        ("genesis", 34, 30) => Some(Verse {
            content: "And Iacob said to Simeon and Leui, Ye haue troubled me to make me to stinke among the inhabitants of the land, amongst the Canaanites, and the Perizzites: and I being few in number, they shall gather themselues together against me, and slay me, and I shal be destroyed, I and my house.",
        }),

        ("genesis", 34, 31) => Some(Verse {
            content: "And they said, Should hee deale with our sister, as with an harlot?",
        }),

        ("genesis", 35, 1) => Some(Verse {
            content: "And God said vnto Iacob, Arise, goe vp to Bethel, and dwel there: and make there an Altar vnto God, that appeared vnto thee, when thou fleddest from the face of Esau thy brother.",
        }),

        ("genesis", 35, 2) => Some(Verse {
            content: "Then Iacob said vnto his household, and to all that were with him, Put away the strange gods that are among you, and bee cleane, and change your garments,",
        }),

        ("genesis", 35, 3) => Some(Verse {
            content: "And let vs arise, and goe vp to Bethel, and I will make there an Altar vnto God, who answered me in the day of my distresse, and was with me in the way which I went.",
        }),

        ("genesis", 35, 4) => Some(Verse {
            content: "And they gaue vnto Iacob all the strange gods which were in their hand, and all their eare-rings which were in their eares, and Iacob hid them vnder the oke which was by Shechem.",
        }),

        ("genesis", 35, 5) => Some(Verse {
            content: "And they iourneyed: and the terrour of God was vpon the cities that were round about them, and they did not pursue after the sonnes of Iacob.",
        }),

        ("genesis", 35, 6) => Some(Verse {
            content: "So Iacob came to Luz, which is in the land of Canaan (that is Bethel) hee and all the people that were with him.",
        }),

        ("genesis", 35, 7) => Some(Verse {
            content: "And hee built there an Altar, and called the place El-Bethel, because there God appeared vnto him, when he fled from the face of his brother.",
        }),

        ("genesis", 35, 8) => Some(Verse {
            content: "But Deborah Rebekahs nurse died, and she was buried beneath Bethel vnder an oke: and the name of it was called Allon Bachuth.",
        }),

        ("genesis", 35, 9) => Some(Verse {
            content: "And God appeared vnto Iacob againe, when he came out of Padan Aram, and blessed him.",
        }),

        ("genesis", 35, 10) => Some(Verse {
            content: "And God said vnto him, Thy name is Iacob: thy name shall not bee called any more Iacob, but Israel shall bee thy name; and hee called his name Israel.",
        }),

        ("genesis", 35, 11) => Some(Verse {
            content: "And God saide vnto him, I am God Almightie: be fruitfull and multiply: a nation and a company of nations shall be of thee, and Kings shall come out of thy loynes.",
        }),

        ("genesis", 35, 12) => Some(Verse {
            content: "And the land which I gaue Abraham, and Isaac, to thee I will giue it, and to thy seed after thee will I giue the land.",
        }),

        ("genesis", 35, 13) => Some(Verse {
            content: "And God went vp from him, in the place where he talked with him.",
        }),

        ("genesis", 35, 14) => Some(Verse {
            content: "And Iacob set vp a pillar in the place where he talked with him, euen a pillar of stone: and hee powred a drinke offering thereon, and he powred oile thereon.",
        }),

        ("genesis", 35, 15) => Some(Verse {
            content: "And Iacob called the name of the place where God spake with him, Bethel.",
        }),

        ("genesis", 35, 16) => Some(Verse {
            content: "And they iourneyed from Bethel: and there was but a litle way to come to Ephrath; and Rachel traueiled, and she had hard labour.",
        }),

        ("genesis", 35, 17) => Some(Verse {
            content: "And it came to passe when shee was in hard labour, that the midwife said vnto her, Feare not: thou shalt haue this sonne also.",
        }),

        ("genesis", 35, 18) => Some(Verse {
            content: "And it came to passe as her soule was in departing, (for she died) that she called his name Ben-oni: but his father called him Beniamin.",
        }),

        ("genesis", 35, 19) => Some(Verse {
            content: "And Rachel died, and was buried in the way to Ephrath, which is Bethlehem.",
        }),

        ("genesis", 35, 20) => Some(Verse {
            content: "And Iacob set a pillar vpon her graue: that is the pillar of Rachels graue vnto this day.",
        }),

        ("genesis", 35, 21) => Some(Verse {
            content: "And Israel iourneyed and spread his tent beyond the towre of Edar.",
        }),

        ("genesis", 35, 22) => Some(Verse {
            content: "And it came to passe when Israel dwelt in that land, that Reuben went & lay with Bilhah his fathers concubine: and Israel heard it. Now the sonnes of Iacob were twelue.",
        }),

        ("genesis", 35, 23) => Some(Verse {
            content: "The sonnes of Leah: Reuben Iacobs first borne, and Simeon, and Leui, and Iudah, and Issachar, and Zebulun.",
        }),

        ("genesis", 35, 24) => Some(Verse {
            content: "The sonnes of Rachel: Ioseph, and Beniamin.",
        }),

        ("genesis", 35, 25) => Some(Verse {
            content: "And the sonnes of Bilhah, Rachels handmaid: Dan and Naphtali.",
        }),

        ("genesis", 35, 26) => Some(Verse {
            content: "And the sonnes of Zilpah, Leahs handmaid: Gad, and Asher. These are the sonnes of Iacob, which were borne to him in Padan Aram.",
        }),

        ("genesis", 35, 27) => Some(Verse {
            content: "And Iacob came vnto Isaac his father vnto Mamre, vnto the citie of Arbah (which is Hebron) where Abraham and Isaac soiourned.",
        }),

        ("genesis", 35, 28) => Some(Verse {
            content: "And the dayes of Isaac were an hundred and fourescore yeeres.",
        }),

        ("genesis", 35, 29) => Some(Verse {
            content: "And Isaac gaue vp the ghost and died, and was gathered vnto his people, being old and full of dayes: and his sonnes Esau and Iacob buried him.",
        }),

        ("genesis", 36, 1) => Some(Verse {
            content: "Now these are the generations of Esau, who is Edom.",
        }),

        ("genesis", 36, 2) => Some(Verse {
            content: "Esau tooke his wiues of the daughters of Canaan: Adah the daughter of Elon the Hittite, and Aholibamah the daughter of Anah the daughter of Zibeon the Hiuite:",
        }),

        ("genesis", 36, 3) => Some(Verse {
            content: "And Bashemath Ishmaels daughter, sister of Nebaioth.",
        }),

        ("genesis", 36, 4) => Some(Verse {
            content: "And Adah bare to Esau, Eliphaz: and Bashemath bare Reuel.",
        }),

        ("genesis", 36, 5) => Some(Verse {
            content: "And Aholibamah bare Ieush, and Iaalam, and Korah: these are the sonnes of Esau, which were borne vnto him in the land of Canaan.",
        }),

        ("genesis", 36, 6) => Some(Verse {
            content: "And Esau tooke his wiues, and his sonnes, and his daughters, and all the persons of his house, and his cattell, and all his beasts, and all his substance, which he had got in the lande of Canaan: and went into the countrey from the face of his brother Iacob.",
        }),

        ("genesis", 36, 7) => Some(Verse {
            content: "For their riches were more then that they might dwell together: and the land wherein they were strangers, could not beare them, because of their cattell.",
        }),

        ("genesis", 36, 8) => Some(Verse {
            content: "Thus dwelt Esau in mount Seir: Esau is Edom.",
        }),

        ("genesis", 36, 9) => Some(Verse {
            content: "And these are the generations of Esau, the father of the Edomites in mount Seir.",
        }),

        ("genesis", 36, 10) => Some(Verse {
            content: "These are the names of Esaus sonnes: Eliphaz the sonne of Adah the wife of Esau, Reuel the sonne of Bashemath, the wife of Esau.",
        }),

        ("genesis", 36, 11) => Some(Verse {
            content: "And the sonnes of Eliphaz were, Teman, Omar, Zepho, and Gatam, and Kenaz.",
        }),

        ("genesis", 36, 12) => Some(Verse {
            content: "And Timna was concubine to Eliphaz Esaus sonne, and shee bare to Eliphaz Amalek: these were the sonnes of Adah Esaus wife.",
        }),

        ("genesis", 36, 13) => Some(Verse {
            content: "And these are the sonnes of Reuel: Nahath and Zerah, Shammah, and Mizzah: these were the sonnes of Bashemath, Esaus wife.",
        }),

        ("genesis", 36, 14) => Some(Verse {
            content: "And these were the sonnes of Aholibamah, the daughter of Anah, daughter of Zibeon Esaus wife: and she bare to Esau, Ieush and Iaalam, and Korah.",
        }),

        ("genesis", 36, 15) => Some(Verse {
            content: "These were dukes of the sonnes of Esau: the sonnes of Eliphaz the first borne sonne of Esau, duke Teman, duke Omar, duke Zepho, duke Kenaz,",
        }),

        ("genesis", 36, 16) => Some(Verse {
            content: "Duke Korah, duke Gatam, and duke Amalek: These are the dukes that came of Eliphaz, in the land of Edom: These were the sonnes of Adah.",
        }),

        ("genesis", 36, 17) => Some(Verse {
            content: "And these are the sonnes of Reuel Esaus sonne: duke Nahath, duke Zerah, duke Shammah, duke Mizzah. These are the dukes that came of Reuel, in the land of Edom: these are the sonnes of Bashemath, Esaus wife.",
        }),

        ("genesis", 36, 18) => Some(Verse {
            content: "And these are the sonnes of Aholibamah Esaus wife: duke Ieush, duke Iaalam, duke Korah: these were the dukes that came of Aholibamah the daughter of Anah Esaus wife.",
        }),

        ("genesis", 36, 19) => Some(Verse {
            content: "These are the sonnes of Esau, (who is Edom) and these are their dukes.",
        }),

        ("genesis", 36, 20) => Some(Verse {
            content: "These are the sonnes of Seir the Horite, who inhabited the land, Lotan, and Shobal, and Zibeon, and Anah.",
        }),

        ("genesis", 36, 21) => Some(Verse {
            content: "And Dishon, and Ezer, and Dishan: these are the dukes of the Horites the children of Seir in the lande of Edom.",
        }),

        ("genesis", 36, 22) => Some(Verse {
            content: "And the children of Lotan, were Hori, and Hemam: and Lotans sister was Timna.",
        }),

        ("genesis", 36, 23) => Some(Verse {
            content: "And the children of Shobal were these: Aluan, and Manahath, and Ebal, Shepho, and Onam.",
        }),

        ("genesis", 36, 24) => Some(Verse {
            content: "And these are the children of Zibeon, both Aiah, and Anah: this was that Anah that found the mules in the wildernesse, as he fed the asses of Zibeon his father.",
        }),

        ("genesis", 36, 25) => Some(Verse {
            content: "And the children of Anah were these: Dishon, and Aholibamah the daughter of Anah.",
        }),

        ("genesis", 36, 26) => Some(Verse {
            content: "And these are the children of Dishon: Hemdan and Eshban, & Ithran, and Cheran.",
        }),

        ("genesis", 36, 27) => Some(Verse {
            content: "The children of Ezer are these: Bilhan and Zaauan, and Akan.",
        }),

        ("genesis", 36, 28) => Some(Verse {
            content: "The children of Dishan are these: Uz, and Aran.",
        }),

        ("genesis", 36, 29) => Some(Verse {
            content: "These are the dukes that came of the Horites: duke Lotan, duke Shobal, duke Zibeon, duke Anah,",
        }),

        ("genesis", 36, 30) => Some(Verse {
            content: "Duke Dishon, duke Ezer, duke Dishan: these are the dukes that came of Hori, among their dukes in the land of Seir.",
        }),

        ("genesis", 36, 31) => Some(Verse {
            content: "And these are the kings that reigned in the land of Edom, before there reigned any king ouer the children of Israel.",
        }),

        ("genesis", 36, 32) => Some(Verse {
            content: "And Bela the sonne of Beor reigned in Edom: and the name of his citie was Dinhabah.",
        }),

        ("genesis", 36, 33) => Some(Verse {
            content: "And Bela died, and Iobab the sonne of Zerah of Bozra reigned in his stead.",
        }),

        ("genesis", 36, 34) => Some(Verse {
            content: "And Iobab died, and Husham of the land of Temani reigned in his stead.",
        }),

        ("genesis", 36, 35) => Some(Verse {
            content: "And Husham died, and Hadad the sonne of Bedad, (who smote Midian in the field of Moab,) reigned in his stead: & the name of his citie was Auith.",
        }),

        ("genesis", 36, 36) => Some(Verse {
            content: "And Hadad died, and Samlah of Masrekah, reigned in his stead.",
        }),

        ("genesis", 36, 37) => Some(Verse {
            content: "And Samlah died, and Saul of Rehoboth, by the riuer, reigned in his stead.",
        }),

        ("genesis", 36, 38) => Some(Verse {
            content: "And Saul died, and Baal-hanan the sonne of Achbor reigned in his stead.",
        }),

        ("genesis", 36, 39) => Some(Verse {
            content: "And Baal-hanan the sonne of Achbor died, and Hadar reigned in his stead: and the name of his citie was Pau, and his wiues name was Mehetabel, the daughter of Matred, the daughter of Mezahab.",
        }),

        ("genesis", 36, 40) => Some(Verse {
            content: "And these are the names of the dukes that came of Esau, according to their families, after their places, by their names: duke Timnah, duke Aluah, duke Ietheth,",
        }),

        ("genesis", 36, 41) => Some(Verse {
            content: "Duke Aholibamah, duke Elah, duke Pinon,",
        }),

        ("genesis", 36, 42) => Some(Verse {
            content: "Duke Kenaz, duke Teman, duke Mibzar,",
        }),

        ("genesis", 36, 43) => Some(Verse {
            content: "Duke Magdiel, duke Iram. These be the dukes of Edom, according to their habitations, in the land of their possession: he is Esau the father of the Edomites.",
        }),

        ("genesis", 37, 1) => Some(Verse {
            content: "And Iacob dwelt in the land wherein his father was a stranger, in the land of Canaan.",
        }),

        ("genesis", 37, 2) => Some(Verse {
            content: "These are the generations of Iacob: Ioseph being seuenteene yeeres old, was feeding the flocke with his brethren, and the lad was with the sonnes of Bilhah, and with the sonnes of Zilpah, his fathers wiues: and Ioseph brought vnto his father their euill report.",
        }),

        ("genesis", 37, 3) => Some(Verse {
            content: "Now Israel loued Ioseph more then all his children, because he was the sonne of his old age: and he made him a coat of many colours.",
        }),

        ("genesis", 37, 4) => Some(Verse {
            content: "And when his brethren saw that their father loued him more then all his brethren, they hated him, and could not speake peaceably vnto him.",
        }),

        ("genesis", 37, 5) => Some(Verse {
            content: "And Ioseph dreamed a dreame, and he told it his brethren, and they hated him yet the more.",
        }),

        ("genesis", 37, 6) => Some(Verse {
            content: "And he said vnto them, Heare, I pray you, this dreame which I haue dreamed.",
        }),

        ("genesis", 37, 7) => Some(Verse {
            content: "For beholde, wee were binding sheaues in the field, and loe, my sheafe arose, and also stood vpright; and behold, your sheaues stood round about, and made obeisance to my sheafe.",
        }),

        ("genesis", 37, 8) => Some(Verse {
            content: "And his brethren saide to him, Shalt thou indeed reigne ouer vs? or shalt thou indeed haue dominion ouer vs? and they hated him yet the more, for his dreames, and for his words.",
        }),

        ("genesis", 37, 9) => Some(Verse {
            content: "And hee dreamed yet another dreame, and told it his brethren, and said, Behold, I haue dreamed a dreame more: and behold, the sunne and the moone, and the eleuen starres made obeisance to me.",
        }),

        ("genesis", 37, 10) => Some(Verse {
            content: "And he told it to his father, and to his brethren: and his father rebuked him, and said vnto him, What is this dreame that thou hast dreamed? shal I, and thy mother, and thy brethren indeed come to bow downe our selues to thee, to the earth?",
        }),

        ("genesis", 37, 11) => Some(Verse {
            content: "And his brethren enuied him: but his father obserued the saying.",
        }),

        ("genesis", 37, 12) => Some(Verse {
            content: "And his brethren went to feed their fathers flocke in Shechem.",
        }),

        ("genesis", 37, 13) => Some(Verse {
            content: "And Israel saide vnto Ioseph, Doe not thy brethren feed the flocke in Shechem? Come, and I will send thee vnto them: & he said to him, Here am I.",
        }),

        ("genesis", 37, 14) => Some(Verse {
            content: "And he said to him, Goe, I pray thee, see whether it bee well with thy brethren, and well with the flockes, and bring me word againe: so hee sent him out of the vale of Hebron, and he came to Shechem.",
        }),

        ("genesis", 37, 15) => Some(Verse {
            content: "And a certaine man found him, and behold, hee was wandring in the field, and the man asked him, saying, What seekest thou?",
        }),

        ("genesis", 37, 16) => Some(Verse {
            content: "And he said, I seeke my brethren: tell me, I pray thee, where they feede their flockes.",
        }),

        ("genesis", 37, 17) => Some(Verse {
            content: "And the man said, They are departed hence: for I heard them say, Let vs goe to Dothan. And Ioseph went after his brethren, and found them in Dothan.",
        }),

        ("genesis", 37, 18) => Some(Verse {
            content: "And when they saw him a farre off, euen before he came neere vnto them, they conspired against him, to slay him.",
        }),

        ("genesis", 37, 19) => Some(Verse {
            content: "And they said one to another, Behold, this dreamer commeth.",
        }),

        ("genesis", 37, 20) => Some(Verse {
            content: "Come now therefore, and let vs slay him, and cast him into some pit, and we will say, Some euill beast hath deuoured him: and we shall see what will become of his dreames.",
        }),

        ("genesis", 37, 21) => Some(Verse {
            content: "And Reuben heard it, and he deliuered him out of their hands, and said; Let vs not kill him.",
        }),

        ("genesis", 37, 22) => Some(Verse {
            content: "And Reuben saide vnto them, Shed no blood, but cast him into this pit that is in the wildernesse, and lay no hand vpon him; that he might rid him out of their hands, to deliuer him to his father againe.",
        }),

        ("genesis", 37, 23) => Some(Verse {
            content: "And it came to passe when Ioseph was come vnto his brethren, that they stript Ioseph out of his coate, his coat of many colours that was on him.",
        }),

        ("genesis", 37, 24) => Some(Verse {
            content: "And they tooke him and cast him into a pit: and the pit was emptie, there was no water in it.",
        }),

        ("genesis", 37, 25) => Some(Verse {
            content: "And they sate downe to eat bread: and they lift vp their eyes and looked, and behold, a company of Ishmeelites came from Gilead, with their camels, bearing spicery, & baulme, and myrrhe, going to cary it downe to Egypt.",
        }),

        ("genesis", 37, 26) => Some(Verse {
            content: "And Iudah saide vnto his brethren, What profit is it if we slay our brother, and conceale his blood?",
        }),

        ("genesis", 37, 27) => Some(Verse {
            content: "Come, and let vs sell him to the Ishmeelites, and let not our hand bee vpon him: for he is our brother, and our flesh; and his brethren were content.",
        }),

        ("genesis", 37, 28) => Some(Verse {
            content: "Then there passed by Midianites merchant men, and they drew and lift vp Ioseph out of the pit, and sold Ioseph to the Ishmeelites for twentie pieces of siluer: and they brought Ioseph into Egypt.",
        }),

        ("genesis", 37, 29) => Some(Verse {
            content: "And Reuben returned vnto the pit, and behold, Ioseph was not in the pit: and he rent his clothes.",
        }),

        ("genesis", 37, 30) => Some(Verse {
            content: "And hee returned vnto his brethren and said, The childe is not, and I, whither shall I goe?",
        }),

        ("genesis", 37, 31) => Some(Verse {
            content: "And they tooke Iosephs coat, and killed a kid of the goats, and dipped the coat in the blood.",
        }),

        ("genesis", 37, 32) => Some(Verse {
            content: "And they sent the coat of many colours, and they brought it to their father, and said, This haue we found: know now whether it bee thy sonnes coat or no.",
        }),

        ("genesis", 37, 33) => Some(Verse {
            content: "And he knew it, and said, It is my sonnes coat: an euil beast hath deuoured him; Ioseph is without doubt rent in pieces.",
        }),

        ("genesis", 37, 34) => Some(Verse {
            content: "And Iacob rent his clothes, and put sackcloth vpon his loines, & mourned for his sonne many dayes.",
        }),

        ("genesis", 37, 35) => Some(Verse {
            content: "And all his sonnes, and all his daughters rose vp to comfort him: but he refused to be comforted: and he said, For I will goe downe into the graue vnto my sonne, mourning; thus his father wept for him.",
        }),

        ("genesis", 37, 36) => Some(Verse {
            content: "And the Medanites sold him into Egypt vnto Potiphar, an officer of Pharaohs, and captaine of the guard.",
        }),

        ("genesis", 38, 1) => Some(Verse {
            content: "And it came to passe at that time, that Iudah went downe from his brethren, and turned in to a certaine Adullamite, whose name was Hirah:",
        }),

        ("genesis", 38, 2) => Some(Verse {
            content: "And Iudah saw there a daughter of a certaine Canaanite, whose name was Shuah: and he tooke her, and went in vnto her.",
        }),

        ("genesis", 38, 3) => Some(Verse {
            content: "And she conceiued & bare a sonne, and he called his name Er.",
        }),

        ("genesis", 38, 4) => Some(Verse {
            content: "And shee conceiued againe, and bare a sonne, and shee called his name, Onan.",
        }),

        ("genesis", 38, 5) => Some(Verse {
            content: "And she yet againe conceiued and bare a sonne, and called his name Shelah: and hee was at Chezib, when shee bare him.",
        }),

        ("genesis", 38, 6) => Some(Verse {
            content: "And Iudah tooke a wife for Er his first borne, whose name was Tamar.",
        }),

        ("genesis", 38, 7) => Some(Verse {
            content: "And Er, Iudahs first borne was wicked in the sight of the LORD, and the LORD slew him.",
        }),

        ("genesis", 38, 8) => Some(Verse {
            content: "And Iudah said vnto Onan, Goe in vnto thy brothers wife, and marrie her, and raise vp seed to thy brother.",
        }),

        ("genesis", 38, 9) => Some(Verse {
            content: "And Onan knew that the seed should not be his; and it came to passe when hee went in vnto his brothers wife, that hee spilled it on the ground, least that hee should giue seed to his brother.",
        }),

        ("genesis", 38, 10) => Some(Verse {
            content: "And the thing which he did, displeased the LORD: wherefore hee slew him also.",
        }),

        ("genesis", 38, 11) => Some(Verse {
            content: "Then said Iudah to Tamar his daughter in law, Remaine a widow at thy fathers house, til Shelah my sonne be growen: (for he said, Lest peraduenture he die also as his brethren did) and Tamar went and dwelt in her fathers house.",
        }),

        ("genesis", 38, 12) => Some(Verse {
            content: "And in processe of time, the daughter of Shuah Iudahs wife died: and Iudah was comforted, and went vp vnto his sheepe-shearers to Timnath, he and his friend Hirah the Adullamite.",
        }),

        ("genesis", 38, 13) => Some(Verse {
            content: "And it was told Lamar, saying, Behold, thy father in law goeth vp to Timnath to sheare his sheepe.",
        }),

        ("genesis", 38, 14) => Some(Verse {
            content: "And shee put her widowes garments off from her, and couered her with a vaile, and wrapped her selfe, and sate in an open place, which is by the way to Timnath: for shee sawe that Shelah was growen, and she was not giuen vnto him to wife.",
        }),

        ("genesis", 38, 15) => Some(Verse {
            content: "When Iudah saw her, he thought her to be an harlot: because she had couered her face.",
        }),

        ("genesis", 38, 16) => Some(Verse {
            content: "And hee turned vnto her by the way, and said, Goe to, I pray thee, let me come in vnto thee: (for he knew not that she was his daughter in law) and she said, what wilt thou giue mee, that thou mayest come in vnto me?",
        }),

        ("genesis", 38, 17) => Some(Verse {
            content: "And hee said, I will send thee a kid from the flocke: and shee saide, Wilt thou giue mee a pledge, till thou send it?",
        }),

        ("genesis", 38, 18) => Some(Verse {
            content: "And he said, What pledge shall I giue thee? And she said, Thy signet, and thy bracelets, and thy staffe, that is in thine hand: and he gaue it her, & came in vnto her, and she conceived by him.",
        }),

        ("genesis", 38, 19) => Some(Verse {
            content: "And shee arose and went away, and laid by her vaile from her, and put on the garments of her widowhood.",
        }),

        ("genesis", 38, 20) => Some(Verse {
            content: "And Iudah sent the kidde by the hand of his friend the Adullamite, to receive his pledge from the womans hand: but he found her not.",
        }),

        ("genesis", 38, 21) => Some(Verse {
            content: "Then hee asked the men of that place, saying, where is the harlot, that was openly by the way side? And they said, There was no harlot in this place.",
        }),

        ("genesis", 38, 22) => Some(Verse {
            content: "And he returned to Iudah, and said, I cannot finde her: and also the men of the place said, That there was no harlot in this place.",
        }),

        ("genesis", 38, 23) => Some(Verse {
            content: "And Iudah said, Let her take it to her, lest we bee shamed: behold, I sent this kidde, and thou hast not found her.",
        }),

        ("genesis", 38, 24) => Some(Verse {
            content: "And it came to passe about three moneths after, that it was tolde Iudah, saying, Tamar thy daughter in law hath played the harlot, and also behold, she is with child by whoredom: and Iudah said, Bring her foorth, and let her be burnt.",
        }),

        ("genesis", 38, 25) => Some(Verse {
            content: "When she was brought forth, she sent to her father in law, saying, By the man whose these are, am I with child: and shee said, Discerne, I pray thee, whose are these, the signet, and bracelets, and staffe.",
        }),

        ("genesis", 38, 26) => Some(Verse {
            content: "And Iudah acknowledged them, and said, She hath bin more righteous then I: because that I gaue her not to Shelah my sonne: and he knew her againe no more.",
        }),

        ("genesis", 38, 27) => Some(Verse {
            content: "And it came to passe in the time of her trauaile, that beholde, twinnes were in her wombe.",
        }),

        ("genesis", 38, 28) => Some(Verse {
            content: "And it came to passe when shee trauailed, that the one put out his hand, and the midwife tooke and bound vpon his hand a skarlet threed, saying, This came out first.",
        }),

        ("genesis", 38, 29) => Some(Verse {
            content: "And it came to passe as he drewe backe his hand, that behold, his brother came out: and she said, how hast thou broken foorth? this breach bee vpon thee: Therefore his name was called Pharez.",
        }),

        ("genesis", 38, 30) => Some(Verse {
            content: "And afterward came out his brother that had the skarlet threed vpon his hand, and his name was called Zarah.",
        }),

        ("genesis", 39, 1) => Some(Verse {
            content: "And Ioseph was brought downe to Egypt, and Potiphar an Officer of Pharaoh, captaine of þe guard, an Egyptian, bought him of the hand of the Ishmeelites, which had brought him downe thither.",
        }),

        ("genesis", 39, 2) => Some(Verse {
            content: "And the LORD was with Ioseph, and hee was a prosperous man, and hee was in the house of his master the Egyptian.",
        }),

        ("genesis", 39, 3) => Some(Verse {
            content: "And his master sawe that the LORD was with him, and that the LORD made all that he did, to prosper in his hand.",
        }),

        ("genesis", 39, 4) => Some(Verse {
            content: "And Ioseph found grace in his sight, and he serued him; and hee made him ouerseer ouer his house, and all that he had he put into his hand.",
        }),

        ("genesis", 39, 5) => Some(Verse {
            content: "And it came to passe from the time that hee had made him overseer in his house, and ouer all that he had, that the LORD blessed the Egyptians house for Iosephs sake: and the blessing of the LORD was vpon all that he had in the house, and in the field.",
        }),

        ("genesis", 39, 6) => Some(Verse {
            content: "And he left all that he had, in Iosephs hand: and he knew not ought he had, saue the bread which he did eate: and Ioseph was a goodly person, and well fauoured.",
        }),

        ("genesis", 39, 7) => Some(Verse {
            content: "And it came to passe after these things, that his masters wife cast her eyes vpon Ioseph, and shee said, Lie with me.",
        }),

        ("genesis", 39, 8) => Some(Verse {
            content: "But he refused, and said vnto his masters wife, Behold, my master wotteth not what is with mee in the house, and he hath committed all that he hath, to my hand.",
        }),

        ("genesis", 39, 9) => Some(Verse {
            content: "There is none greater in this house then I: neither hath hee kept backe any thing from me, but thee, because thou art his wife: how then can I doe this great wickednesse, and sinne against God?",
        }),

        ("genesis", 39, 10) => Some(Verse {
            content: "And it came to passe as she spake to Ioseph day by day, that hee hearkened not vnto her, to lie by her, or to bee with her.",
        }),

        ("genesis", 39, 11) => Some(Verse {
            content: "And it came to passe about this time, that Ioseph went in to the house, to doe his busines, and there was none of the men of the house there within.",
        }),

        ("genesis", 39, 12) => Some(Verse {
            content: "And shee caught him by his garment, saying, Lie with me: and he left his garment in her hand, and fled, and got him out.",
        }),

        ("genesis", 39, 13) => Some(Verse {
            content: "And it came to passe, when she saw that hee had left his garment in her hand, and was fled forth;",
        }),

        ("genesis", 39, 14) => Some(Verse {
            content: "That she called vnto the men of her house, and spake vnto them, saying, See, he hath brought in an Hebrew vnto vs, to mocke vs: he came in vnto me to lie with me, and I cried with a loud voice.",
        }),

        ("genesis", 39, 15) => Some(Verse {
            content: "And it came to passe, when hee heard that I lifted vp my voice, and cried, that he left his garment with mee, and fled, and got him out.",
        }),

        ("genesis", 39, 16) => Some(Verse {
            content: "And she laid vp his garment by her, vntill her lord came home.",
        }),

        ("genesis", 39, 17) => Some(Verse {
            content: "And she spake vnto him, according to these words, saying, The Hebrew seruant which thou hast brought vnto vs, came in vnto me to mocke me.",
        }),

        ("genesis", 39, 18) => Some(Verse {
            content: "And it came to passe as I lift vp my voice, and cried, that he left his garment with me, and fled out.",
        }),

        ("genesis", 39, 19) => Some(Verse {
            content: "And it came to passe when his master heard the words of his wife, which she spake vnto him, saying, After this maner did thy seruant to me, that his wrath was kindled.",
        }),

        ("genesis", 39, 20) => Some(Verse {
            content: "And Iosephs master tooke him, and put him into the prison, a place, where þe kings prisoners were bound: and he was there in the prison.",
        }),

        ("genesis", 39, 21) => Some(Verse {
            content: "But the LORD was with Ioseph, and shewed him mercie, and gaue him fauour in the sight of the keeper of the prison.",
        }),

        ("genesis", 39, 22) => Some(Verse {
            content: "And the keeper of the prison committed to Iosephs hand all the prisoners that were in the prison, and whatsoeuer they did there, he was the doer of it:",
        }),

        ("genesis", 39, 23) => Some(Verse {
            content: "The keeper of the prison looked not to any thing, that was vnder his hand, because the LORD was with him: & that which he did, the LORD made it to prosper.",
        }),

        ("genesis", 40, 1) => Some(Verse {
            content: "And it came to passe after these things, that the Butler of the King of Egypt, and his Baker, had offended their lord the King of Egypt.",
        }),

        ("genesis", 40, 2) => Some(Verse {
            content: "And Pharaoh was wroth against two of his officers, against the chiefe of the Butlers, and against the chiefe of the Bakers.",
        }),

        ("genesis", 40, 3) => Some(Verse {
            content: "And he put them in ward in the house of the captaine of the guard, into the prison, the place where Ioseph was bound.",
        }),

        ("genesis", 40, 4) => Some(Verse {
            content: "And the captaine of the guard charged Ioseph with them, and he serued them, and they continued a season in warde.",
        }),

        ("genesis", 40, 5) => Some(Verse {
            content: "And they dreamed a dreame both of them, each man his dreame in one night, each man according to the interpretation of his dreame, the Butler and the Baker of the king of Egypt, which were bound in the prison.",
        }),

        ("genesis", 40, 6) => Some(Verse {
            content: "And Ioseph came in vnto them in the morning, and looked vpon them, and behold, they were sad.",
        }),

        ("genesis", 40, 7) => Some(Verse {
            content: "And he asked Pharaohs officers that were with him in the warde of his lords house, saying, wherefore looke ye so sadly to day?",
        }),

        ("genesis", 40, 8) => Some(Verse {
            content: "And they said vnto him, we haue dreamed a dreame, and there is no interpreter of it. And Ioseph said vnto them, Doe not interpretations belong to God? tell me them, I pray you.",
        }),

        ("genesis", 40, 9) => Some(Verse {
            content: "And the chiefe Butler tolde his dreame to Ioseph, and said to him; In my dreame, beholde, a vine was before mee:",
        }),

        ("genesis", 40, 10) => Some(Verse {
            content: "And in the vine were three branches, and it was as though it budded, and her blossoms shot foorth; and the clusters thereof brought forth ripe grapes.",
        }),

        ("genesis", 40, 11) => Some(Verse {
            content: "And Pharaohs cup was in my hand, and I tooke the grapes and pressed them into Pharaohs cup: and I gaue the cup into Pharaohs hand.",
        }),

        ("genesis", 40, 12) => Some(Verse {
            content: "And Ioseph said vnto him, This is the interpretation of it: the three branches are three dayes,",
        }),

        ("genesis", 40, 13) => Some(Verse {
            content: "Yet within three dayes shall Pharaoh lift vp thine head, and restore thee vnto thy place, and thou shalt deliuer Pharaohs cup into his hand, after the former manner when thou wast his Butler.",
        }),

        ("genesis", 40, 14) => Some(Verse {
            content: "But thinke on me, when it shall be well with thee, and shew kindenesse, I pray thee, vnto mee, and make mention of me vnto Pharaoh, and bring me out of this house.",
        }),

        ("genesis", 40, 15) => Some(Verse {
            content: "For indeed I was stollen away out of the land of the Hebrewes: and here also haue I done nothing, that they should put me into the dungeon.",
        }),

        ("genesis", 40, 16) => Some(Verse {
            content: "When the chiefe Baker saw, that the interpretation was good, he said vnto Ioseph, I also was in my dreame, and behold, I had three white baskets on my head.",
        }),

        ("genesis", 40, 17) => Some(Verse {
            content: "And in the vppermost basket there was of all maner of bake-meats for Pharaoh, and the birds did eat them out of the basket vpon my head.",
        }),

        ("genesis", 40, 18) => Some(Verse {
            content: "And Ioseph answered, and said, This is the interpretation thereof: the three baskets are three dayes:",
        }),

        ("genesis", 40, 19) => Some(Verse {
            content: "Yet within three dayes shall Pharaoh lift vp thy head from off thee, and shall hang thee on a tree, and the birds shall eate thy flesh from off thee.",
        }),

        ("genesis", 40, 20) => Some(Verse {
            content: "And it came to passe the third day, which was Pharaohs birth day, that hee made a feast vnto all his seruaunts: and he lifted vp by the head of the chiefe Butler, and of the chiefe Baker among his seruants.",
        }),

        ("genesis", 40, 21) => Some(Verse {
            content: "And he restored the chiefe Butler vnto his Butlership againe, and hee gaue the cup into Pharaohs hand.",
        }),

        ("genesis", 40, 22) => Some(Verse {
            content: "But he hanged the chiefe Baker, as Ioseph had interpreted to them.",
        }),

        ("genesis", 40, 23) => Some(Verse {
            content: "Yet did not the chiefe Butler remember Ioseph, but forgate him.",
        }),

        ("genesis", 41, 1) => Some(Verse {
            content: "And it came to passe at the end of two ful yeeres, that Pharaoh dreamed: and beholde, hee stood by the riuer.",
        }),

        ("genesis", 41, 2) => Some(Verse {
            content: "And behold, there came vp out of the riuer seuen well fauoured kine, and fat fleshed, and they fed in a medow.",
        }),

        ("genesis", 41, 3) => Some(Verse {
            content: "And behold, seuen other kine came vp after them out of the riuer, ill fauoured and leane fleshed, and stood by the other kine, vpon the brinke of the riuer.",
        }),

        ("genesis", 41, 4) => Some(Verse {
            content: "And the ill fauoured and leane fleshed kine, did eate vp the seuen well fauoured and fat kine: So Pharaoh awoke.",
        }),

        ("genesis", 41, 5) => Some(Verse {
            content: "And hee slept and dreamed the second time: and beholde, seuen eares of corne came vp vpon one stalke, ranke and good.",
        }),

        ("genesis", 41, 6) => Some(Verse {
            content: "And beholde, seuen thinne eares and blasted with the Eastwind, sprang vp after them.",
        }),

        ("genesis", 41, 7) => Some(Verse {
            content: "And the seuen thinne eares deuoured the seuen ranke and full eares: and Pharaoh awoke, and behold, it was a dreame.",
        }),

        ("genesis", 41, 8) => Some(Verse {
            content: "And it came to passe in the morning, that his spirit was troubled, and he sent and called for all the Magicians of Egypt, and all the wise men thereof: and Pharaoh tolde them his dreame; but there was none that could interprete them vnto Pharaoh.",
        }),

        ("genesis", 41, 9) => Some(Verse {
            content: "Then spake the chiefe Butler vnto Pharaoh, saying, I doe remember my faults this day.",
        }),

        ("genesis", 41, 10) => Some(Verse {
            content: "Pharaoh was wroth with his seruants, and put mee in warde, in the captaine of the guards house, both mee, and the chiefe Baker.",
        }),

        ("genesis", 41, 11) => Some(Verse {
            content: "And we dreamed a dreame in one night, I and he: we dreamed each man according to the interpretation of his dreame.",
        }),

        ("genesis", 41, 12) => Some(Verse {
            content: "And there was there with vs a yong man an Hebrew, seruant to the captaine of the guard: and wee told him, and he interpreted to vs our dreames, to each man according to his dreame, he did interpret.",
        }),

        ("genesis", 41, 13) => Some(Verse {
            content: "And it came to passe, as he interpreted to vs, so it was; mee he restored vnto mine office, and him he hanged.",
        }),

        ("genesis", 41, 14) => Some(Verse {
            content: "Then Pharaoh sent and called Ioseph, and they brought him hastily out of the dungeon: And he shaued himselfe, and changed his raiment, and came in vnto Pharaoh.",
        }),

        ("genesis", 41, 15) => Some(Verse {
            content: "And Pharaoh said vnto Ioseph, I haue dreamed a dreame, and there is none that can interpret it: and I haue heard say of thee, that thou canst vnderstand a dreame, to interpret it.",
        }),

        ("genesis", 41, 16) => Some(Verse {
            content: "And Ioseph answered Pharaoh, saying; It is not in me: God shall giue Pharaoh an answere of peace.",
        }),

        ("genesis", 41, 17) => Some(Verse {
            content: "And Pharaoh said vnto Ioseph; In my dreame, behold, I stood vpon the banke of the riuer.",
        }),

        ("genesis", 41, 18) => Some(Verse {
            content: "And behold, there came vp out of the riuer seuen kine, fat fleshed and well fauoured, and they fed in a medow.",
        }),

        ("genesis", 41, 19) => Some(Verse {
            content: "And behold, seuen other kine came vp after them, poore and very ill fauoured, and leane fleshed, such as I neuer saw in all the land of Egypt for badnes.",
        }),

        ("genesis", 41, 20) => Some(Verse {
            content: "And the leane, & the ill fauoured kine, did eate vp the first seuen fat kine.",
        }),

        ("genesis", 41, 21) => Some(Verse {
            content: "And when they had eaten them vp, it could not bee knowen that they had eaten them, but they were still ill fauoured, as at the beginning: So I awoke.",
        }),

        ("genesis", 41, 22) => Some(Verse {
            content: "And I saw in my dreame, and behold, seuen eares came vp in one stalke, full and good.",
        }),

        ("genesis", 41, 23) => Some(Verse {
            content: "And behold, seuen eares withered, thin & blasted with the East wind, sprung vp after them.",
        }),

        ("genesis", 41, 24) => Some(Verse {
            content: "And the thin eares deuoured the seuen good eares: and I told this vnto the magicians, but there was none that could declare it to me.",
        }),

        ("genesis", 41, 25) => Some(Verse {
            content: "And Ioseph said vnto Pharaoh, the dreame of Pharaoh is one; God hath shewed Pharaoh what he is about to doe.",
        }),

        ("genesis", 41, 26) => Some(Verse {
            content: "The seuen good kine are seuen yeeres: and the seuen good eares are seuen yeeres: the dreame is one.",
        }),

        ("genesis", 41, 27) => Some(Verse {
            content: "And the seuen thin and ill fauoured kine that came vp after them, are seuen yeeres: and the seuen emptie eares blasted with the East wind, shall bee seuen yeeres of famine.",
        }),

        ("genesis", 41, 28) => Some(Verse {
            content: "This is the thing which I haue spoken vnto Pharaoh: what God is about to doe, he sheweth vnto Pharaoh.",
        }),

        ("genesis", 41, 29) => Some(Verse {
            content: "Behold, there come seuen yeeres of great plentie, throughout all the land of Egypt.",
        }),

        ("genesis", 41, 30) => Some(Verse {
            content: "And there shall arise after them, seuen yeeres of famine, and all the plentie shall be forgotten in the land of Egypt: and the famine shall consume the land.",
        }),

        ("genesis", 41, 31) => Some(Verse {
            content: "And the plentie shal not be knowen in the land, by reason of that famine following: for it shalbe very grieuous.",
        }),

        ("genesis", 41, 32) => Some(Verse {
            content: "And for that the dreame was doubled vnto Pharaoh twice, it is because the thing is established by God: and God will shortly bring it to passe.",
        }),

        ("genesis", 41, 33) => Some(Verse {
            content: "Now therfore let Pharaoh looke out a man discreet and wise, and set him ouer the land of Egypt.",
        }),

        ("genesis", 41, 34) => Some(Verse {
            content: "Let Pharaoh doe this, and let him appoint officers ouer the land, & take vp the fift part of the land of Egypt, in the seuen plenteous yeeres.",
        }),

        ("genesis", 41, 35) => Some(Verse {
            content: "And let them gather all the food of those good yeeres that come, and lay vp corne vnder the hand of Pharaoh, and let them keepe food in the cities.",
        }),

        ("genesis", 41, 36) => Some(Verse {
            content: "And that food shall be for store to the land, against the seuen yeeres of famine, which shall bee in the land of Egypt, that the land perish not through the famine.",
        }),

        ("genesis", 41, 37) => Some(Verse {
            content: "And the thing was good in the eyes of Pharaoh, and in the eyes of all his seruants.",
        }),

        ("genesis", 41, 38) => Some(Verse {
            content: "And Pharaoh said vnto his seruants, Can we find such a one, as this is, a man in whom the spirit of God is?",
        }),

        ("genesis", 41, 39) => Some(Verse {
            content: "And Pharaoh said vnto Ioseph, Forasmuch as God hath shewed thee all this, there is none so discreete and wise, as thou art:",
        }),

        ("genesis", 41, 40) => Some(Verse {
            content: "Thou shalt be ouer my house, and according vnto thy word shall all my people be ruled: only in the throne will I be greater then thou.",
        }),

        ("genesis", 41, 41) => Some(Verse {
            content: "And Pharaoh said vnto Ioseph, See, I haue set thee ouer all the land of Egypt.",
        }),

        ("genesis", 41, 42) => Some(Verse {
            content: "And Pharaoh tooke off his ring from his hand, & put it vpon Iosephs hand, and arayed him in vestures of fine linnen, and put a gold chaine about his necke.",
        }),

        ("genesis", 41, 43) => Some(Verse {
            content: "And he made him to ride in the second charet which he had: and they cried before him, Bow the knee: and he made him ruler ouer all the land of Egypt.",
        }),

        ("genesis", 41, 44) => Some(Verse {
            content: "And Pharaoh said vnto Ioseph, I am Pharaoh, and without thee shall no man lift vp his hand or foote, in all the land of Egypt.",
        }),

        ("genesis", 41, 45) => Some(Verse {
            content: "And Pharaoh called Iosephs name, Zaphnath-Paaneah, and he gaue him to wife Asenath the daughter of Poti-pherah, priest of On: and Ioseph went out ouer all the lande of Egypt.",
        }),

        ("genesis", 41, 46) => Some(Verse {
            content: "And Ioseph was thirtie yeeres old when he stood before Pharaoh king of Egypt) and Ioseph went out from the presence of Pharaoh, and went thorowout all the land of Egypt.",
        }),

        ("genesis", 41, 47) => Some(Verse {
            content: "And in the seuen plenteous yeres the earth brought forth by handfuls.",
        }),

        ("genesis", 41, 48) => Some(Verse {
            content: "And he gathered vp all the foode of the seuen yeeres, which were in the land of Egypt, and laid vp the foode in the cities: the foode of the field which was round about euery citie, laid he vp in the same.",
        }),

        ("genesis", 41, 49) => Some(Verse {
            content: "And Ioseph gathered corne as the sand of the sea, very much, vntill he left numbring: for it was without number.",
        }),

        ("genesis", 41, 50) => Some(Verse {
            content: "And vnto Ioseph were borne two sonnes, before the yeeres of famine came: which Asenath the daughter of Poti-pherah, Priest of On bare vnto him.",
        }),

        ("genesis", 41, 51) => Some(Verse {
            content: "And Ioseph called the name of the first borne Manasseh: for God, said hee, hath made me forget all my toile, and all my fathers house.",
        }),

        ("genesis", 41, 52) => Some(Verse {
            content: "And the name of the second called he Ephraim: for God hath caused mee to be fruitfull in the land of my affliction.",
        }),

        ("genesis", 41, 53) => Some(Verse {
            content: "And the seuen yeeres of plenteousnesse, that was in the land of Egypt, were ended.",
        }),

        ("genesis", 41, 54) => Some(Verse {
            content: "And the seuen yeeres of dearth beganne to come according as Ioseph had saide, and the dearth was in all lands: but in all the land of Egypt there was bread.",
        }),

        ("genesis", 41, 55) => Some(Verse {
            content: "And when all the land of Egypt was famished, the people cried to Pharaoh for bread: and Pharaoh said vnto all the Egyptians, Goe vnto Ioseph: what he saith to you, doe.",
        }),

        ("genesis", 41, 56) => Some(Verse {
            content: "And the famine was ouer all the face of the earth; and Ioseph opened all the storehouses, and solde vnto the Egyptians: and the famine waxed sore in the land of Egypt.",
        }),

        ("genesis", 41, 57) => Some(Verse {
            content: "And all countreys came into Egypt to Ioseph, for to buy corne, because that the famine was so sore in all lands.",
        }),

        ("genesis", 42, 1) => Some(Verse {
            content: "Now when Iacob saw that there was corne in Egypt, Iacob said vnto his sonnes, Why doe ye looke one vpon an other?",
        }),

        ("genesis", 42, 2) => Some(Verse {
            content: "And hee said, Beholde, I haue heard that there is corne in Egypt: get you downe thither and buy for vs from thence, that we may liue, and not die.",
        }),

        ("genesis", 42, 3) => Some(Verse {
            content: "And Iosephs ten brethren went downe to buy corne in Egypt.",
        }),

        ("genesis", 42, 4) => Some(Verse {
            content: "But Beniamin Iosephs brother, Iacob sent not with his brethren: for he said, Lest peraduenture mischiefe befall him.",
        }),

        ("genesis", 42, 5) => Some(Verse {
            content: "And the sonnes of Israel came to buy corne among those that came: for the famine was in the land of Canaan.",
        }),

        ("genesis", 42, 6) => Some(Verse {
            content: "And Ioseph was the gouernour ouer the land, and hee it was that sold to all the people of the land: and Iosephs brethren came, & bowed downe themselues before him, with their faces to the earth.",
        }),

        ("genesis", 42, 7) => Some(Verse {
            content: "And Ioseph saw his brethren, and he knew them, but made himselfe strange vnto them, and spake roughly vnto them; and hee saide vnto them, Whence come ye? And they said, From the land of Canaan, to buy food.",
        }),

        ("genesis", 42, 8) => Some(Verse {
            content: "And Ioseph knew his brethren, but they knew not him.",
        }),

        ("genesis", 42, 9) => Some(Verse {
            content: "And Ioseph remembred the dreames which hee dreamed of them, and said vnto them, Ye are spies: to see the nakednes of the land you are come.",
        }),

        ("genesis", 42, 10) => Some(Verse {
            content: "And they said vnto him, Nay, my lord, but to buy food are thy seruants come.",
        }),

        ("genesis", 42, 11) => Some(Verse {
            content: "We are all one mans sonnes, we are true men: thy seruants are no spies.",
        }),

        ("genesis", 42, 12) => Some(Verse {
            content: "And he said vnto them, Nay: but to see the nakednesse of the land, you are come.",
        }),

        ("genesis", 42, 13) => Some(Verse {
            content: "And they said, Thy seruants are twelue brethren, the sonnes of one man in the land of Canaan: and behold, the yongest is this day with our father, and one is not.",
        }),

        ("genesis", 42, 14) => Some(Verse {
            content: "And Ioseph said vnto them, That is it that I spake vnto you, saying, Ye are spies.",
        }),

        ("genesis", 42, 15) => Some(Verse {
            content: "Hereby ye shall be proued: by the life of Pharaoh ye shall not goe foorth hence, except your yongest brother come hither.",
        }),

        ("genesis", 42, 16) => Some(Verse {
            content: "Send one of you, and let him fetch your brother, and ye shalbe kept in prison, that your wordes may be proued, whether there be any trueth in you: or els by the life of Pharaoh surely ye are spies.",
        }),

        ("genesis", 42, 17) => Some(Verse {
            content: "And he put them all together into warde, three dayes.",
        }),

        ("genesis", 42, 18) => Some(Verse {
            content: "And Ioseph said vnto them the third day, This doe, and liue: for I feare God.",
        }),

        ("genesis", 42, 19) => Some(Verse {
            content: "If ye be true men, let one of your brethren be bound in the house of your prison: goe ye, carry corne for the famine of your houses.",
        }),

        ("genesis", 42, 20) => Some(Verse {
            content: "But bring your yongest brother vnto mee, so shall your wordes be verified, and yee shall not die: and they did so.",
        }),

        ("genesis", 42, 21) => Some(Verse {
            content: "And they said one to another, We are verily guiltie concerning our brother, in that we saw the anguish of his soule, when he besought vs, and we would not heare: therefore is this distresse come vpon vs.",
        }),

        ("genesis", 42, 22) => Some(Verse {
            content: "And Reuben answered them, saying, Spake I not vnto you, saying, Doe not sinne against the childe, and ye would not heare? therefore behold also, his blood is required.",
        }),

        ("genesis", 42, 23) => Some(Verse {
            content: "And they knew not that Ioseph vnderstood them: for hee spake vnto them by an interpreter.",
        }),

        ("genesis", 42, 24) => Some(Verse {
            content: "And hee turned himselfe about from them and wept, and returned to them againe, and communed with them, and tooke from them Simeon, and bound him before their eyes.",
        }),

        ("genesis", 42, 25) => Some(Verse {
            content: "Then Ioseph commanded to fill their sackes with corne, and to restore euery mans money into his sacke, and to giue them prouision for the way: and thus did he vnto them.",
        }),

        ("genesis", 42, 26) => Some(Verse {
            content: "And they laded their asses with the corne, and departed thence.",
        }),

        ("genesis", 42, 27) => Some(Verse {
            content: "And as one of them opened his sacke, to giue his asse prouender in the Inne, he espied his money: for behold, it was in his sackes mouth.",
        }),

        ("genesis", 42, 28) => Some(Verse {
            content: "And he said vnto his brethren, My money is restored, and loe, it is euen in my sacke: and their heart failed them, and they were afraid, saying one to an other, What is this that God hath done vnto vs?",
        }),

        ("genesis", 42, 29) => Some(Verse {
            content: "And they came vnto Iacob their father, vnto the land of Canaan, and told him all that befell vnto them, saying;",
        }),

        ("genesis", 42, 30) => Some(Verse {
            content: "The man who is the lord of the land, spake roughly to vs, and tooke vs for spies of the countrey.",
        }),

        ("genesis", 42, 31) => Some(Verse {
            content: "And we said vnto him, We are true men; we are no spies.",
        }),

        ("genesis", 42, 32) => Some(Verse {
            content: "We be twelue brethren, sonnes of our father: one is not, and the yongest is this day with our father, in the land of Canaan.",
        }),

        ("genesis", 42, 33) => Some(Verse {
            content: "And the man the lord of the countrey said vnto vs, Hereby shall I know that ye are true men: leaue one of your brethren here with me, and take foode for the famine of your housholds, and be gone.",
        }),

        ("genesis", 42, 34) => Some(Verse {
            content: "And bring your yongest brother vnto me: then shall I know that you are no spies, but that you are true men: so will I deliuer you your brother, and ye shall traffique in the land.",
        }),

        ("genesis", 42, 35) => Some(Verse {
            content: "And it came to passe as they emptied their sacks, that behold, euery mans bundle of money was in his sacke: and when both they and their father saw the bundels of money, they were afraid.",
        }),

        ("genesis", 42, 36) => Some(Verse {
            content: "And Iacob their father said vnto them, We haue ye bereaued of my children: Ioseph is not, and Simeon is not, and ye wil take Beniamin away: all these things are against me.",
        }),

        ("genesis", 42, 37) => Some(Verse {
            content: "And Reuben spake vnto his father, saying; Slay my two sonnes, if I bring him not to thee: deliuer him into my hand, and I will bring him to thee againe.",
        }),

        ("genesis", 42, 38) => Some(Verse {
            content: "And he said, My sonne shall not goe downe with you, for his brother is dead, and he is left alone: if mischiefe befall him by the way in the which yee goe, then shall ye bring downe my gray haires with sorrow to the graue.",
        }),

        ("genesis", 43, 1) => Some(Verse {
            content: "And the famine was sore in the land.",
        }),

        ("genesis", 43, 2) => Some(Verse {
            content: "And it came to passe when they had eaten vp the corne, which they had brought out of Egypt, their father said vnto them, Goe againe, buy vs a little foode.",
        }),

        ("genesis", 43, 3) => Some(Verse {
            content: "And Iudah spake vnto him, saying, The man did solemnly protest vnto vs, saying, Ye shall not see my face, except your brother be with you.",
        }),

        ("genesis", 43, 4) => Some(Verse {
            content: "If thou wilt send our brother with vs, we will goe downe and buy thee food.",
        }),

        ("genesis", 43, 5) => Some(Verse {
            content: "But if thou wilt not send him, we will not goe downe: for the man saide vnto vs, Ye shall not see my face, except your brother be with you.",
        }),

        ("genesis", 43, 6) => Some(Verse {
            content: "And Israel said, Wherefore dealt ye so ill with me, as to tell the man whether ye had yet a brother?",
        }),

        ("genesis", 43, 7) => Some(Verse {
            content: "And they said, The man asked vs straitly of our state, and of our kindred, saying, Is your father yet aliue? haue yee another brother? and we tolde him according to the tenour of these words: Could we certainely knowe that he would say, Bring your brother downe?",
        }),

        ("genesis", 43, 8) => Some(Verse {
            content: "And Iudah said vnto Israel his father, Send the lad with me, and wee will arise and go, that we may liue, and not die, both we, and thou, and also our little ones.",
        }),
        ("genesis", 43, 9) => Some(Verse {
            content: "I will be surety for him; of my hand shalt thou require him: if I bring him not vnto thee, and set him before thee, then let me beare the blame for euer.",
        }),

        ("genesis", 43, 10) => Some(Verse {
            content: "For except we had lingred, surely now wee had returned this second time.",
        }),

        ("genesis", 43, 11) => Some(Verse {
            content: "And their father Israel said vnto them, If it must bee so now, doe this: take of the best fruits in the land in your vessels, and carie downe the man a Present, a litle balme, and a litle honie, spices, and myrrhe, nuts, and almonds.",
        }),

        ("genesis", 43, 12) => Some(Verse {
            content: "And take double money in your hand, and the money that was brought againe in the mouth of your sackes: carie it againe in your hand, peraduenture it was an ouersight.",
        }),

        ("genesis", 43, 13) => Some(Verse {
            content: "Take also your brother, and arise, goe againe vnto the man.",
        }),

        ("genesis", 43, 14) => Some(Verse {
            content: "And God Almightie giue you mercie before the man, that he may send away your other brother, and Beniamin: If I be bereaued of my children, I am bereaued.",
        }),

        ("genesis", 43, 15) => Some(Verse {
            content: "And the men tooke that Present, and they tooke double money in their hand, and Beniamin, and rose vp, and went downe to Egypt, and stood before Ioseph.",
        }),

        ("genesis", 43, 16) => Some(Verse {
            content: "And when Ioseph sawe Beniamin with them, hee said to the ruler of his house, Bring these men home, and slay, and make ready: for these men shall dine with me at noone.",
        }),

        ("genesis", 43, 17) => Some(Verse {
            content: "And the man did as Ioseph hade: and the man brought the men into Iosephs house.",
        }),

        ("genesis", 43, 18) => Some(Verse {
            content: "And the men were afraid, because they were brought into Iosephs house, and they said, Because of the money that was returned in our sackes at the first time are we brought in, that hee may seeke occasion against vs, and fall vpon vs, and take vs for bondmen, and our asses.",
        }),

        ("genesis", 43, 19) => Some(Verse {
            content: "And they came neere to the steward of Iosephs house, and they communed with him at the doore of the house,",
        }),

        ("genesis", 43, 20) => Some(Verse {
            content: "And said, O Sir, we came indeed downe at the first time to buy food.",
        }),

        ("genesis", 43, 21) => Some(Verse {
            content: "And it came to passe when wee came to the Inne, that wee opened our sackes, and behold, euery mans money was in the mouth of his sacke, our money in ful weight: and we haue brought it againe in our hand.",
        }),

        ("genesis", 43, 22) => Some(Verse {
            content: "And other money haue wee brought downe in our handes to buy food: we cannot tell who put our money in our sackes.",
        }),

        ("genesis", 43, 23) => Some(Verse {
            content: "And he said, Peace be to you, feare not: your God, and the God of your father, hath giuen you treasure in your sackes: I had your money. And hee brought Simeon out vnto them.",
        }),

        ("genesis", 43, 24) => Some(Verse {
            content: "And the man brought the men into Iosephs house, and gaue them water, and they washed their feete, and he gaue their asses prouender.",
        }),

        ("genesis", 43, 25) => Some(Verse {
            content: "And they made ready the Present against Ioseph came at noone: for they heard that they should eate bread there.",
        }),

        ("genesis", 43, 26) => Some(Verse {
            content: "And when Ioseph came home, they brought him the Present which was in their hand, into the house, and bowed themselues to him to the earth.",
        }),

        ("genesis", 43, 27) => Some(Verse {
            content: "And he asked them of their welfare, and said, Is your father well, the old man of whom ye spake? Is he yet aliue?",
        }),

        ("genesis", 43, 28) => Some(Verse {
            content: "And they answered, Thy seruant our father is in good health, hee is yet aliue: & they bowed downe their heads, and made obeisance.",
        }),

        ("genesis", 43, 29) => Some(Verse {
            content: "And he lift vp his eyes, and sawe his brother Beniamin, his mothers sonne, and said, Is this your yonger brother, of whom yee spake vnto mee? and he said, God be gracious vnto thee, my sonne.",
        }),

        ("genesis", 43, 30) => Some(Verse {
            content: "And Ioseph made haste: for his bowels did yerne vpon his brother: and he sought where to weepe, and hee entred into his chamber, & wept there.",
        }),

        ("genesis", 43, 31) => Some(Verse {
            content: "And he washed his face, and went out, and refrained himselfe, and saide, Set on bread.",
        }),

        ("genesis", 43, 32) => Some(Verse {
            content: "And they set on for him by himselfe, and for them by themselues, and for the Egyptians which did eate with him, by themselues: because the Egyptians might not eate bread with the Hebrewes: for that is an abomination vnto the Egyptians.",
        }),

        ("genesis", 43, 33) => Some(Verse {
            content: "And they sate before him, the first borne according to his birthright, and the yongest according to his youth: and the men marueiled one at another.",
        }),

        ("genesis", 43, 34) => Some(Verse {
            content: "And hee tooke and sent measses vnto them from before him: but Beniamins measse was fiue times so much as any of theirs: and they drunke, and were merry with him.",
        }),

        ("genesis", 44, 1) => Some(Verse {
            content: "And hee commaunded the steward of his house, saying, Fill the mens sackes with food, as much as they can carie, and put euery mans money in his sacks mouth.",
        }),

        ("genesis", 44, 2) => Some(Verse {
            content: "And put my cup, the siluer cup, in the sackes mouth of the youngest, and his corne money: and he did according to the word that Ioseph had spoken.",
        }),

        ("genesis", 44, 3) => Some(Verse {
            content: "Assoone as the morning was light, the men were sent away, they, and their asses.",
        }),

        ("genesis", 44, 4) => Some(Verse {
            content: "And when they were gone out of the citie, and not yet farre off, Ioseph said vnto his steward, Up, follow after the men; and when thou doest ouertake them, say vnto them, Wherefore haue ye rewarded euill for good?",
        }),

        ("genesis", 44, 5) => Some(Verse {
            content: "Is not this it, in which my lord drinketh? and whereby indeed he diuineth? ye haue done euill in so doing.",
        }),

        ("genesis", 44, 6) => Some(Verse {
            content: "And he ouertooke them, and he spake vnto them these same words.",
        }),

        ("genesis", 44, 7) => Some(Verse {
            content: "And they said vnto him, Wherefore saith my lord these words? God forbid that thy seruants should doe according to this thing.",
        }),

        ("genesis", 44, 8) => Some(Verse {
            content: "Behold, the money which wee found in our sackes mouthes, wee brought againe vnto thee, out of the land of Canaan: how then should wee steale out of thy lords house, siluer or golde?",
        }),

        ("genesis", 44, 9) => Some(Verse {
            content: "With whom soeuer of thy seruants it be found, both let him die, and we also will be my lords bondmen.",
        }),

        ("genesis", 44, 10) => Some(Verse {
            content: "And he said, Now also let it be according vnto your wordes: hee with whom it is found, shall be my seruant: and ye shall be blamelesse.",
        }),

        ("genesis", 44, 11) => Some(Verse {
            content: "Then they speedily tooke downe euery man his sacke to the ground, and opened euery man his sacke.",
        }),

        ("genesis", 44, 12) => Some(Verse {
            content: "And he searched, and began at the eldest, and left at the yongest: and the cup was found in Beniamins sacke.",
        }),

        ("genesis", 44, 13) => Some(Verse {
            content: "Then they rent their clothes, and laded euery man his asse, and returned to the citie.",
        }),

        ("genesis", 44, 14) => Some(Verse {
            content: "And Iudah and his brethren came to Iosephs house: (for he was yet there) and they fell before him on the ground.",
        }),

        ("genesis", 44, 15) => Some(Verse {
            content: "And Ioseph said vnto them, What deed is this that ye haue done? wote ye not, that such a man as I can certainely diuine?",
        }),

        ("genesis", 44, 16) => Some(Verse {
            content: "And Iudah said, What shall wee say vnto my lord? what shal we speake? or how shall we cleare our selues? God hath found out the iniquitie of thy seruants: beholde, wee are my lords seruants, both we, and he also with whom the cup is found.",
        }),

        ("genesis", 44, 17) => Some(Verse {
            content: "And he said, God forbid that I should doe so: but the man in whose hand the cup is found, he shal be my seruant; and as for you, get you vp in peace vnto your father.",
        }),

        ("genesis", 44, 18) => Some(Verse {
            content: "Then Iudah came neere vnto him, and said, Oh my lord, let thy seruant, I pray thee, speake a word in my lords eares, & let not thine anger burne against thy seruant: for thou art euen as Pharaoh.",
        }),

        ("genesis", 44, 19) => Some(Verse {
            content: "My lord asked his seruants, saying; Haue ye a father, or a brother?",
        }),

        ("genesis", 44, 20) => Some(Verse {
            content: "And we said vnto my lord, Wee haue a father, an olde man, and a childe of his old age, a little one: and his brother is dead, and he alone is left of his mother, and his father loueth him.",
        }),

        ("genesis", 44, 21) => Some(Verse {
            content: "And thou saidst vnto thy seruants, Bring him downe vnto mee, that I may set mine eyes vpon him.",
        }),

        ("genesis", 44, 22) => Some(Verse {
            content: "And we said vnto my lord, The lad cannot leaue his father: for if hee should leaue his father, his father would die.",
        }),

        ("genesis", 44, 23) => Some(Verse {
            content: "And thou saidst vnto thy seruants, Except your yongest brother come downe with you, you shall see my face no more.",
        }),

        ("genesis", 44, 24) => Some(Verse {
            content: "And it came to passe when wee came vp vnto thy seruant my father, we told him the words of my lord.",
        }),

        ("genesis", 44, 25) => Some(Verse {
            content: "And our father said, Goe againe, and buy vs a little food.",
        }),

        ("genesis", 44, 26) => Some(Verse {
            content: "And we saide, Wee cannot goe downe: if our yongest brother be with vs, then will we goe downe: for wee may not see the mans face, except our yongest brother be with vs.",
        }),

        ("genesis", 44, 27) => Some(Verse {
            content: "And thy seruant my father said vnto vs, Ye know that my wife bare me two sonnes.",
        }),

        ("genesis", 44, 28) => Some(Verse {
            content: "And the one went out from me, and I said, Surely he is torne in pieces: and I saw him not since.",
        }),

        ("genesis", 44, 29) => Some(Verse {
            content: "And if ye take this also from me, and mischiefe befall him, ye shall bring downe my gray haires with sorrow to the graue.",
        }),

        ("genesis", 44, 30) => Some(Verse {
            content: "Now therefore when I come to thy seruant my father, and the lad bee not with vs; (seeing that his life is bound vp in the lads life.)",
        }),

        ("genesis", 44, 31) => Some(Verse {
            content: "It shall come to passe, when he seeth that the lad is not with vs, that he will die, and thy seruants shall bring downe the gray haires of thy seruant our father with sorrow to the graue.",
        }),

        ("genesis", 44, 32) => Some(Verse {
            content: "For thy seruant became surety for the lad vnto my father, saying, If I bring him not vnto thee, then I shall beare the blame to my father, for euer.",
        }),

        ("genesis", 44, 33) => Some(Verse {
            content: "Now therefore, I pray thee, let thy seruant abide in stead of the lad, a bondman to my lord, and let the lad goe vp with his brethren.",
        }),

        ("genesis", 44, 34) => Some(Verse {
            content: "For how shall I goe vp to my father, and the lad be not with mee, lest peraduenture I see the euill that shall come on my father?",
        }),

        ("genesis", 45, 1) => Some(Verse {
            content: "Then Ioseph could not refraine himselfe before all them that stood by him: and he cried, Cause euery man to goe out from me; and there stood no man with him, while Ioseph made himselfe knowen vnto his brethren.",
        }),

        ("genesis", 45, 2) => Some(Verse {
            content: "And he wept aloud: and the Egyptians, and the house of Pharaoh heard.",
        }),

        ("genesis", 45, 3) => Some(Verse {
            content: "And Ioseph said vnto his brethren, I am Ioseph; Doeth my father yet liue? and his brethren could not answere him: for they were troubled at his presence.",
        }),

        ("genesis", 45, 4) => Some(Verse {
            content: "And Ioseph said vnto his brethren, Come neere to me, I pray you: and they came neere; and he said, I am Ioseph your brother, whom ye sold into Egypt.",
        }),

        ("genesis", 45, 5) => Some(Verse {
            content: "Now therefore bee not grieued, nor angry with your selues, that yee sold me hither: for God did send me before you, to preserue life.",
        }),

        ("genesis", 45, 6) => Some(Verse {
            content: "For these two yeeres hath the famine bene in the land: and yet there are fiue yeeres, in the which there shall neither be earing nor haruest.",
        }),

        ("genesis", 45, 7) => Some(Verse {
            content: "And God sent me before you, to preserue you a posteritie in the earth, and to saue your liues by a great deliuerance.",
        }),

        ("genesis", 45, 8) => Some(Verse {
            content: "So now it was not you that sent me hither, but God: and he hath made me a father to Pharaoh, and lord of all his house, and a ruler throughout all the land of Egypt.",
        }),

        ("genesis", 45, 9) => Some(Verse {
            content: "Haste you, and goe vp to my father, and say vnto him, Thus saith thy sonne Ioseph; God hath made me lord of all Egypt; come downe vnto me, tary not.",
        }),

        ("genesis", 45, 10) => Some(Verse {
            content: "And thou shalt dwell in the land of Goshen, and thou shalt be neere vnto me, thou, and thy children, and thy childrens children, and thy flockes, and thy heards, and all that thou hast.",
        }),

        ("genesis", 45, 11) => Some(Verse {
            content: "And there wil I nourish thee, (for yet there are fiue yeeres of famine) lest thou and thy houshold, and all that thou hast, come to pouertie.",
        }),

        ("genesis", 45, 12) => Some(Verse {
            content: "And behold, your eyes see, and the eyes of my brother Beniamin, that it is my mouth that speaketh vnto you.",
        }),

        ("genesis", 45, 13) => Some(Verse {
            content: "And you shall tell my father of all my glory in Egypt, and of all that you haue seene, and ye shall haste, and bring downe my father hither.",
        }),

        ("genesis", 45, 14) => Some(Verse {
            content: "And he fel vpon his brother Beniamins necke, and wept: and Beniamin wept vpon his necke.",
        }),

        ("genesis", 45, 15) => Some(Verse {
            content: "Moreouer hee kissed all his brethren, and wept vpon them: and after that, his brethren talked with him.",
        }),

        ("genesis", 45, 16) => Some(Verse {
            content: "And the fame thereof was heard in Pharaohs house, saying, Iosephs brethren are come: and it pleased Pharaoh well, and his seruants.",
        }),

        ("genesis", 45, 17) => Some(Verse {
            content: "And Pharaoh said vnto Ioseph, Say vnto thy brethren, This doe yee, lade your beasts and goe, get you vnto the land of Canaan.",
        }),

        ("genesis", 45, 18) => Some(Verse {
            content: "And take your father, and your housholds, and come vnto mee: and I wil giue you the good of the land of Egypt, and ye shall eat the fat of the land.",
        }),

        ("genesis", 45, 19) => Some(Verse {
            content: "Now thou art commanded, this doe yee; Take you wagons out of the land of Egypt for your little ones, and for your wiues, and bring your father, and come.",
        }),

        ("genesis", 45, 20) => Some(Verse {
            content: "Also regard not your stuffe: for the good of all the land of Egypt is yours.",
        }),

        ("genesis", 45, 21) => Some(Verse {
            content: "And the children of Israel did so: and Ioseph gaue them wagons, according to the commandement of Pharaoh, and gaue them prouision for the way.",
        }),

        ("genesis", 45, 22) => Some(Verse {
            content: "To all of them he gaue each man changes of raiment: but to Beniamin hee gaue three hundred pieces of siluer, and fiue changes of raiment.",
        }),

        ("genesis", 45, 23) => Some(Verse {
            content: "And to his father hee sent after this maner: ten asses laden with the good things of Egypt, and ten shee asses laden with corne, and bread and meat for his father by the way.",
        }),

        ("genesis", 45, 24) => Some(Verse {
            content: "So he sent his brethren away, and they departed: and hee said vnto them, See that yee fall not out by the way.",
        }),

        ("genesis", 45, 25) => Some(Verse {
            content: "And they went vp out of Egypt, and came into the land of Canaan vnto Iacob their father,",
        }),

        ("genesis", 45, 26) => Some(Verse {
            content: "And told him, saying, Ioseph is yet aliue, and he is gouernour ouer all the land of Egypt. And Iacobs heart fainted, for he beleeued them not.",
        }),

        ("genesis", 45, 27) => Some(Verse {
            content: "And they told him all the words of Ioseph, which hee had saide vnto them: and when hee saw the wagons which Ioseph had sent to carie him, the spirit of Iacob their father reuiued.",
        }),

        ("genesis", 45, 28) => Some(Verse {
            content: "And Israel said, It is enough; Ioseph my sonne is yet aliue: I will goe and see him before I die.",
        }),

        ("genesis", 46, 1) => Some(Verse {
            content: "And Israel tooke his iourney with all that hee had, and came to Beersheba, and offered sacrifices vnto the God of his father Isaac.",
        }),

        ("genesis", 46, 2) => Some(Verse {
            content: "And God spake vnto Israel in the visions of the night, and said, Iacob, Iacob. And he said, Here am I.",
        }),

        ("genesis", 46, 3) => Some(Verse {
            content: "And he said, I am God, the God of thy father, feare not to goe downe into Egypt: for I will there make of thee a great nation.",
        }),

        ("genesis", 46, 4) => Some(Verse {
            content: "I will goe downe with thee into Egypt; and I will also surely bring thee vp againe: and Ioseph shall put his hand vpon thine eyes.",
        }),

        ("genesis", 46, 5) => Some(Verse {
            content: "And Iacob rose vp from Beersheba: and the sonnes of Israel caried Iacob their father, and their litle ones, and their wiues, in the wagons which Pharaoh had sent to cary him.",
        }),

        ("genesis", 46, 6) => Some(Verse {
            content: "And they tooke their cattell, and their goods which they had gotten in the land of Canaan, and came into Egypt, Iacob, and all his seed with him:",
        }),

        ("genesis", 46, 7) => Some(Verse {
            content: "His sonnes, and his sonnes sonnes with him, his daughters, and his sonnes daughters, and all his seed brought he with him into Egypt.",
        }),

        ("genesis", 46, 8) => Some(Verse {
            content: "And these are the names of the children of Israel, which came into Egypt, Iacob and his sonnes: Reuben Iacobs first borne;",
        }),

        ("genesis", 46, 9) => Some(Verse {
            content: "And the sonnes of Reuben, Hanoch, and Phallu, and Hezron, and Carmi.",
        }),

        ("genesis", 46, 10) => Some(Verse {
            content: "And the sonnes of Simeon: Iemuel, and Iamin, and Ohad, and Iachin, and Zohar, and Shaul the sonne of a Canaanitish woman.",
        }),

        ("genesis", 46, 11) => Some(Verse {
            content: "And the sonnes of Leui: Gershon, Kohath, and Merari.",
        }),

        ("genesis", 46, 12) => Some(Verse {
            content: "And the sonnes of Iudah: Er, and Onan, and Shelah, and Pharez, and Zerah: But Er & Onan died in the land of Canaan. And the sonnes of Pharez, were Hezron, and Hamul.",
        }),

        ("genesis", 46, 13) => Some(Verse {
            content: "And the sonnes of Issachar: Tola, and Phuuah, and Iob, and Shimron.",
        }),

        ("genesis", 46, 14) => Some(Verse {
            content: "And the sonnes of Zebulun: Sered, and Elon, and Iahleel.",
        }),

        ("genesis", 46, 15) => Some(Verse {
            content: "These bee the sonnes of Leah, which she bare vnto Iacob in Padan-Aram, with his daughter Dinah: all the soules of his sonnes and his daughters, were thirtie and three.",
        }),

        ("genesis", 46, 16) => Some(Verse {
            content: "And the sonnes of Gad: Ziphion, and Haggi, Shuni, and Ezbon, Eri, and Arodi, and Areli.",
        }),

        ("genesis", 46, 17) => Some(Verse {
            content: "And the sonnes of Asher: Iimnah, and Ishuah, and Isui, and Beriah, and Serah their sister: And the sonnes of Beriah: Heber, and Malchiel.",
        }),

        ("genesis", 46, 18) => Some(Verse {
            content: "These are the sonnes of Zilpah, whome Laban gaue to Leah his daughter: and these she bare vnto Iacob, euen sixteene soules.",
        }),

        ("genesis", 46, 19) => Some(Verse {
            content: "The sonnes of Rachel Iacobs wife: Ioseph, and Beniamin.",
        }),

        ("genesis", 46, 20) => Some(Verse {
            content: "And vnto Ioseph in the lande of Egypt, were borne Manasseh and Ephraim, which Asenath the daughter of Poti-pherah Priest of On bare vnto him.",
        }),

        ("genesis", 46, 21) => Some(Verse {
            content: "And the sonnes of Beniamin were Belah, and Becher, and Ashbel, Gera, and Naaman, Ehi and Rosh, Muppim, and Huppim, and Ard.",
        }),

        ("genesis", 46, 22) => Some(Verse {
            content: "These are the sonnes of Rachel which were borne to Iacob: all the soules were fourteene.",
        }),

        ("genesis", 46, 23) => Some(Verse {
            content: "And the sonnes of Dan: Hushim.",
        }),

        ("genesis", 46, 24) => Some(Verse {
            content: "And the sonnes of Naphtali: Iahzeel, and Guni, and Iezer, and Shillem.",
        }),

        ("genesis", 46, 25) => Some(Verse {
            content: "These are the sonnes of Bilhah, which Laban gaue vnto Rachel his daughter, and she bare these vnto Iacob: all the soules were seuen.",
        }),

        ("genesis", 46, 26) => Some(Verse {
            content: "All the soules that came with Iacob into Egypt, which came out of his loines, besides Iacobs sonnes wiues, all the soules were threescore and sixe.",
        }),

        ("genesis", 46, 27) => Some(Verse {
            content: "And the sonnes of Ioseph, which were borne him in Egypt, were two soules: all the soules of the house of Iacob, which came into Egypt, were threescore and ten.",
        }),

        ("genesis", 46, 28) => Some(Verse {
            content: "And he sent Iudah before him vnto Ioseph, to direct his face vnto Goshen, and they came into the lande of Goshen.",
        }),

        ("genesis", 46, 29) => Some(Verse {
            content: "And Ioseph made ready his charet, and went vp to meet Israel his father, to Goshen, and presented himselfe vnto him: and he fell on his necke, and wept on his necke a good while.",
        }),

        ("genesis", 46, 30) => Some(Verse {
            content: "And Israel saide vnto Ioseph, Now let me die, since I haue seene thy face, because thou art yet aliue.",
        }),

        ("genesis", 46, 31) => Some(Verse {
            content: "And Ioseph said vnto his brethren, and vnto his fathers house, I will goe vp, and shew Pharaoh, and say vnto him, My brethren, & my fathers house, which were in the land of Canaan, are come vnto me.",
        }),

        ("genesis", 46, 32) => Some(Verse {
            content: "And the men are sheapheards, for their trade hath bene to feed cattell: and they haue brought their flocks, and their heards, and all that they haue.",
        }),

        ("genesis", 46, 33) => Some(Verse {
            content: "And it shall come to passe when Pharaoh shall call you, and shall say, What is your occupation?",
        }),

        ("genesis", 46, 34) => Some(Verse {
            content: "That ye shall say, Thy seruants trade hath bene about cattell, from our youth euen vntill now, both we, and also our fathers: that ye may dwell in the land of Goshen; for euery shepheard is an abomination vnto the Egyptians.",
        }),

        ("genesis", 47, 1) => Some(Verse {
            content: "Then Ioseph came and tolde Pharaoh, and saide, My father and my brethren, and their flockes, and their heards, and all that they haue, are come out of the land of Canaan: and behold, they are in the land of Goshen.",
        }),

        ("genesis", 47, 2) => Some(Verse {
            content: "And hee tooke some of his brethren, euen fiue men, & presented them vnto Pharaoh.",
        }),

        ("genesis", 47, 3) => Some(Verse {
            content: "And Pharaoh said vnto his brethren, What is your occupation? And they said vnto Pharaoh, Thy seruants are shepheards, both wee and also our fathers.",
        }),

        ("genesis", 47, 4) => Some(Verse {
            content: "They said moreouer vnto Pharaoh, For to soiourne in the land are we come: for thy seruants haue no pasture for their flockes, for the famine is sore in the land of Canaan: now therefore we pray thee, let thy seruants dwel in the land of Goshen.",
        }),

        ("genesis", 47, 5) => Some(Verse {
            content: "And Pharaoh spake vnto Ioseph, saying, Thy father and thy brethren are come vnto thee.",
        }),

        ("genesis", 47, 6) => Some(Verse {
            content: "The land of Egypt is before thee: in the best of the land make thy father and brethren to dwell, in the lande of Goshen let them dwell: and if thou knowest any man of actiuitie amongst them, then make them rulers ouer my cattell.",
        }),

        ("genesis", 47, 7) => Some(Verse {
            content: "And Ioseph brought in Iacob his father, and set him before Pharaoh: and Iacob blessed Pharaoh.",
        }),

        ("genesis", 47, 8) => Some(Verse {
            content: "And Pharaoh said vnto Iacob, How old art thou?",
        }),

        ("genesis", 47, 9) => Some(Verse {
            content: "And Iacob said vnto Pharaoh, The dayes of the yeeres of my pilgrimage are an hundred & thirtie yeres: few and euill haue the dayes of the yeeres of my life bene, and haue not attained vnto the dayes of the yeeres of the life of my fathers, in the dayes of their pilgrimage.",
        }),

        ("genesis", 47, 10) => Some(Verse {
            content: "And Iacob blessed Pharaoh, and went out from before Pharaoh.",
        }),

        ("genesis", 47, 11) => Some(Verse {
            content: "And Ioseph placed his father, and his brethren, and gaue them a possession in the land of Egypt, in the best of the land, in the land of Rameses, as Pharaoh had commanded.",
        }),

        ("genesis", 47, 12) => Some(Verse {
            content: "And Ioseph nourished his father and his brethren, and all his fathers houshold with bread, according to their families.",
        }),

        ("genesis", 47, 13) => Some(Verse {
            content: "And there was no bread in all the land: for the famine was very sore, so that the land of Egypt and all the land of Canaan fainted by reason of the famine.",
        }),

        ("genesis", 47, 14) => Some(Verse {
            content: "And Ioseph gathered vp all the money that was found in the land of Egypt, and in the land of Canaan, for the corne which they bought: and Ioseph brought the money into Pharaohs house.",
        }),

        ("genesis", 47, 15) => Some(Verse {
            content: "And when money failed in the land of Egypt, and in the land of Canaan, all the Egyptians came vnto Ioseph, and said, Giue vs bread: for why should we die in thy presence? for the money faileth.",
        }),

        ("genesis", 47, 16) => Some(Verse {
            content: "And Ioseph said, Giue your cattell: and I will giue you for your catell, if money faile.",
        }),

        ("genesis", 47, 17) => Some(Verse {
            content: "And they brought their cattel vnto Ioseph: and Ioseph gaue them bread in exchange for horses, and for the flockes, and for the cattell of the heards, and for the asses, and he fed them with bread, for all their cattel, for that yeere.",
        }),

        ("genesis", 47, 18) => Some(Verse {
            content: "When that yeere was ended, they came vnto him the second yeere, and said vnto him, We will not hide it from my lord, how that our money is spent, my lord also had our heards of cattell: there is not ought left in the sight of my lord, but our bodies, and our lands.",
        }),

        ("genesis", 47, 19) => Some(Verse {
            content: "Wherfore shall we die before thine eyes, both we, and our land? buy vs and our land for bread, and we and our land will be seruants vnto Pharaoh: and giue vs seede that we may liue and not die, that the land be not desolate.",
        }),

        ("genesis", 47, 20) => Some(Verse {
            content: "And Ioseph bought all the land of Egypt for Pharaoh: for the Egyptians sold euery man his field, because the famine preuailed ouer them: so the land became Pharaohs.",
        }),

        ("genesis", 47, 21) => Some(Verse {
            content: "And as for the people, he remoued them to cities from one end of the borders of Egypt, euen to the other ende thereof.",
        }),

        ("genesis", 47, 22) => Some(Verse {
            content: "Onely the land of the Priests bought he not: for the priests had a portion assigned them of Pharaoh, and did eate their portion which Pharaoh gaue them: wherefore they solde not their lands.",
        }),

        ("genesis", 47, 23) => Some(Verse {
            content: "Then Ioseph said vnto the people, Behold, I haue bought you this day, and your land for Pharaoh: Loe, here is seed for you, and ye shall sow the land.",
        }),

        ("genesis", 47, 24) => Some(Verse {
            content: "And it shall come to passe in the increase, that you shall giue the fift part vnto Pharaoh, and foure parts shall be your owne, for seed of the field, and for your food, and for them of your households, and for food for your litle ones.",
        }),

        ("genesis", 47, 25) => Some(Verse {
            content: "And they said, Thou hast saued our liues: let vs find grace in the sight of my lord, and we will be Pharaohs seruants.",
        }),

        ("genesis", 47, 26) => Some(Verse {
            content: "And Ioseph made it a law ouer the land of Egypt vnto this day, that Pharaoh should haue the fift part: except the land of the priests onely, which became not Pharaohs.",
        }),

        ("genesis", 47, 27) => Some(Verse {
            content: "And Israel dwelt in the land of Egypt in the countrey of Goshen, and they had possessions therein, and grew, and multiplied exceedingly.",
        }),

        ("genesis", 47, 28) => Some(Verse {
            content: "And Iacob liued in the land of Egypt seuenteene yeres: so the whole age of Iacob was an hundred fourtie and seuen yeeres.",
        }),

        ("genesis", 47, 29) => Some(Verse {
            content: "And the time drew nigh that Israel must die, and he called his sonne Ioseph, and said vnto him, If now I haue found grace in thy sight, put, I pray thee, thy hand vnder my thigh, and deale kindly and truely with mee, bury me not, I pray thee, in Egypt.",
        }),

        ("genesis", 47, 30) => Some(Verse {
            content: "But I will lie with my fathers, and thou shalt carie mee out of Egypt, and bury me in their burying place: and he said, I will doe as thou hast said.",
        }),

        ("genesis", 47, 31) => Some(Verse {
            content: "And he said, Sweare vnto mee: and he sware vnto him. And Israel bowed himselfe vpon the beds head.",
        }),

        ("genesis", 48, 1) => Some(Verse {
            content: "And it came to passe after these things, that one told Ioseph, Behold, thy father is sicke: and he tooke with him his two sonnes, Manasseh and Ephraim.",
        }),

        ("genesis", 48, 2) => Some(Verse {
            content: "And one told Iacob, and said, Behold, thy sonne Ioseph commeth vnto thee: and Israel strengthened himselfe, and sate vpon the bed.",
        }),

        ("genesis", 48, 3) => Some(Verse {
            content: "And Iacob saide vnto Ioseph, God Almightie appeared vnto mee at Luz in the land of Canaan, and blessed mee,",
        }),

        ("genesis", 48, 4) => Some(Verse {
            content: "And said vnto me, Behold, I wil make thee fruitfull, and multiplie thee, and I will make of thee a multitude of people, and will giue this land to thy seede after thee, for an euerlasting possession.",
        }),

        ("genesis", 48, 5) => Some(Verse {
            content: "And now thy two sonnes, Ephraim and Manasseh, which were borne vnto thee in the land of Egypt, before I came vnto thee into Egypt, are mine: as Reuben and Simeon, they shalbe mine.",
        }),

        ("genesis", 48, 6) => Some(Verse {
            content: "And thy issue which thou begettest after them, shall be thine, and shall be called after the name of their brethren in their inheritance.",
        }),

        ("genesis", 48, 7) => Some(Verse {
            content: "And as for me, when I came from Padan, Rachel died by me in the land of Canaan, in the way, when yet there was but a little way to come vnto Ephrath: and I buried her there in the way of Ephrath, the same is Bethlehem.",
        }),

        ("genesis", 48, 8) => Some(Verse {
            content: "And Israel behelde Iosephs sonnes, and said, Who are these?",
        }),

        ("genesis", 48, 9) => Some(Verse {
            content: "And Ioseph said vnto his father, They are my sonnes, whom God hath giuen me in this place: and he said, Bring them, I pray thee, vnto me, and I will blesse them.",
        }),

        ("genesis", 48, 10) => Some(Verse {
            content: "(Now the eyes of Israel were dimme for age, so that he could not see,) and hee brought them neere vnto him, and he kissed them, and imbraced them.",
        }),

        ("genesis", 48, 11) => Some(Verse {
            content: "And Israel said vnto Ioseph, I had not thought to see thy face: and loe, God hath shewed me also thy seed.",
        }),

        ("genesis", 48, 12) => Some(Verse {
            content: "And Ioseph brought them out from betweene his knees, and hee bowed himselfe with his face to the earth.",
        }),

        ("genesis", 48, 13) => Some(Verse {
            content: "And Ioseph tooke them both, Ephraim in his right hand, toward Israels left hand, and Manasseh in his left hand towards Israels right hand, and brought them neere vnto him.",
        }),

        ("genesis", 48, 14) => Some(Verse {
            content: "And Israel stretched out his right hand, and layd it vpon Ephraims head who was the yonger; and his left hand vpon Manassehs head, guiding his hands wittingly: for Manasseh was the first borne.",
        }),

        ("genesis", 48, 15) => Some(Verse {
            content: "And he blessed Ioseph and said, God before whom my fathers Abraham and Isaac did walke, the God which fedde mee all my life long vnto this day,",
        }),

        ("genesis", 48, 16) => Some(Verse {
            content: "The Angel which redeemed mee from all euill, blesse the laddes, and let my name be named on them, and the name of my fathers Abraham and Isaac, and let them grow into a multitude in the midst of the earth.",
        }),

        ("genesis", 48, 17) => Some(Verse {
            content: "And when Ioseph saw that his father laide his right hand vpon the head of Ephraim, it displeased him: and he held vp his fathers hand, to remoue it from Ephraims head, vnto Manassehs head.",
        }),

        ("genesis", 48, 18) => Some(Verse {
            content: "And Ioseph saide vnto his father, Not so my father: for this is the first borne; put thy right hand vpon his head.",
        }),

        ("genesis", 48, 19) => Some(Verse {
            content: "And his father refused, and said, I know it, my sonne, I know it: he also shall become a people, and he also shall be great: but truely his yonger brother shall be greater then he; and his seede shall become a multitude of nations.",
        }),

        ("genesis", 48, 20) => Some(Verse {
            content: "And he blessed them that day, saying, In thee shall Israel blesse, saying, God make thee as Ephraim, and as Manasseh: and he set Ephraim before Manasseh.",
        }),

        ("genesis", 48, 21) => Some(Verse {
            content: "And Israel saide vnto Ioseph, Behold, I die: but God shall be with you, and bring you againe vnto the land of your fathers.",
        }),

        ("genesis", 48, 22) => Some(Verse {
            content: "Moreouer I haue giuen to thee one portion aboue thy brethren, which I tooke out of the hand of the Amorite with my sword, and with my bow.",
        }),

        ("genesis", 49, 1) => Some(Verse {
            content: "And Iacob called vnto his sonnes, and said, Gather your selues together, that I may tell you that which shall befall you in the last dayes.",
        }),

        ("genesis", 49, 2) => Some(Verse {
            content: "Gather your selues together, and heare ye sonnes of Iacob, and hearken vnto Israel your father.",
        }),

        ("genesis", 49, 3) => Some(Verse {
            content: "Reuben, thou art my first borne, my might, and the beginning of my strength, the excellencie of dignitie, and the excellencie of power:",
        }),

        ("genesis", 49, 4) => Some(Verse {
            content: "Unstable as water, thou shalt not excell, because thou wentest vp to thy fathers bed: then defiledst thou it. He went vp to my couche.",
        }),

        ("genesis", 49, 5) => Some(Verse {
            content: "Simeon and Leui are brethren, instruments of crueltie are in their habitations.",
        }),

        ("genesis", 49, 6) => Some(Verse {
            content: "O my soule, come not thou into their secret: vnto their assembly mine honour be not thou vnited: for in their anger they slew a man, and in their selfe will they digged downe a wall.",
        }),

        ("genesis", 49, 7) => Some(Verse {
            content: "Cursed be their anger, for it was fierce; and their wrath, for it was cruell: I will diuide them in Iacob, and scatter them in Israel.",
        }),

        ("genesis", 49, 8) => Some(Verse {
            content: "Iudah, thou art he whom thy brethren shall praise: thy hand shall be in the necke of thine enemies, thy fathers children shall bow downe before thee.",
        }),

        ("genesis", 49, 9) => Some(Verse {
            content: "Iudah is a Lyons whelpe: from the pray my sonne thou art gone vp: he stouped downe, hee couched as a Lyon, and as an old Lyon: who shall rouse him vp?",
        }),

        ("genesis", 49, 10) => Some(Verse {
            content: "The scepter shall not depart from Iudah, nor a Law-giuer from betweene his feete, vntill Shiloh come: and vnto him shall the gathering of the people be:",
        }),

        ("genesis", 49, 11) => Some(Verse {
            content: "Binding his foale vnto the vine, and his asses colt vnto the choice vine; he washed his garments in wine, and his clothes in the blood of grapes.",
        }),

        ("genesis", 49, 12) => Some(Verse {
            content: "His eyes shall be red with wine, and his teeth white with milke.",
        }),

        ("genesis", 49, 13) => Some(Verse {
            content: "Zebulun shall dwell at the hauen of the sea, and hee shall be for an Hauen of ships: and his border shall be vnto Zidon.",
        }),

        ("genesis", 49, 14) => Some(Verse {
            content: "Issachar is a strong asse, couching downe betweene two burdens.",
        }),

        ("genesis", 49, 15) => Some(Verse {
            content: "And he saw that rest was good, and the land that it was pleasant: and bowed his shoulder to beare, and became a seruant vnto tribute.",
        }),

        ("genesis", 49, 16) => Some(Verse {
            content: "Dan shall iudge his people, as one of the tribes of Israel.",
        }),

        ("genesis", 49, 17) => Some(Verse {
            content: "Dan shalbe a serpent by the way, an adder in the path, that biteth the horse heeles, so that his rider shall fall backward.",
        }),

        ("genesis", 49, 18) => Some(Verse {
            content: "I haue waited for thy saluation, O LORD.",
        }),

        ("genesis", 49, 19) => Some(Verse {
            content: "Gad, a troupe shall ouercome him: but he shall ouercome at the last.",
        }),

        ("genesis", 49, 20) => Some(Verse {
            content: "Out of Asher his bread shall be fat, and he shall yeeld royall dainties.",
        }),

        ("genesis", 49, 21) => Some(Verse {
            content: "Naphtali is a hinde let loose: He giueth goodly words.",
        }),

        ("genesis", 49, 22) => Some(Verse {
            content: "Ioseph is a fruitfull bough, euen a fruitfull bough by a well, whose branches runne ouer the wall.",
        }),

        ("genesis", 49, 23) => Some(Verse {
            content: "The archers haue sorely grieued him, and shot at him, and hated him.",
        }),

        ("genesis", 49, 24) => Some(Verse {
            content: "But his bow abode in strength, and the armes of his hands were made strong, by the hands of the mighty God of Iacob: from thence is the Sheapheard, the stone of Israel,",
        }),

        ("genesis", 49, 25) => Some(Verse {
            content: "Euen by the God of thy father who shall helpe thee, and by the Almightie, who shall blesse thee with blessings of heauen aboue, blessings of the deepe that lyeth vnder, blessings of the breasts and of the wombe.",
        }),

        ("genesis", 49, 26) => Some(Verse {
            content: "The blessings of thy father haue preuailed aboue the blessings of my progenitors: vnto the vtmost bound of the euerlasting hils, they shall bee on the head of Ioseph, and on the crowne of the head of him that was separate from his brethren.",
        }),

        ("genesis", 49, 27) => Some(Verse {
            content: "Beniamin shall rauine as a wolfe: In the morning hee shall deuoure the pray, and at night he shall diuide the spoile.",
        }),

        ("genesis", 49, 28) => Some(Verse {
            content: "All these are the twelue tribes of Israel, and this is it that their father spake vnto them, and blessed them: euery one according to his blessing he blessed them.",
        }),

        ("genesis", 49, 29) => Some(Verse {
            content: "And hee charged them and said vnto them, I am to bee gathered vnto my people: burie me with my fathers, in the caue that is in the field of Ephron the Hittite,",
        }),

        ("genesis", 49, 30) => Some(Verse {
            content: "In the caue that is in the field of Machpelah, which is before Mamre, in the land of Canaan, which Abraham bought with the field of Ephron the Hittite, for a possession of a burying place.",
        }),

        ("genesis", 49, 31) => Some(Verse {
            content: "(There they buried Abraham and Sarah his wife, there they buried Isaac and Rebekah his wife, and there I buried Leah.)",
        }),

        ("genesis", 49, 32) => Some(Verse {
            content: "The purchase of the field and of the caue that is therein, was from the children of Heth.",
        }),

        ("genesis", 49, 33) => Some(Verse {
            content: "And when Iacob had made an end of commanding his sonnes, he gathered vp his feete into the bed, and yeelded vp the ghost, and was gathered vnto his people.",
        }),

        ("genesis", 50, 1) => Some(Verse {
            content: "And Ioseph fell vpon his fathers face, and wept vpon him, and kissed him.",
        }),

        ("genesis", 50, 2) => Some(Verse {
            content: "And Ioseph commanded his seruants the physicians to imbalme his father: and the physicians imbalmed Israel.",
        }),

        ("genesis", 50, 3) => Some(Verse {
            content: "And fortie dayes were fulfilled for him, (for so are fulfilled the dayes of those which are imbalmed) and the Egyptians mourned for him threescore and ten dayes.",
        }),

        ("genesis", 50, 4) => Some(Verse {
            content: "And when the dayes of his mourning were past, Ioseph spake vnto the house of Pharaoh, saying, If now I haue found grace in your eyes, speake, I pray you, in the eares of Pharaoh, saying,",
        }),

        ("genesis", 50, 5) => Some(Verse {
            content: "My father made me sweare, saying, Loe, I die: in my graue which I haue digged for me, in the land of Canaan, there shalt thou bury me. Now therfore let me goe vp, I pray thee, and bury my father, and I will come againe.",
        }),

        ("genesis", 50, 6) => Some(Verse {
            content: "And Pharaoh said, Goe vp, and bury thy father, according as he made thee sweare.",
        }),

        ("genesis", 50, 7) => Some(Verse {
            content: "And Ioseph went vp to bury his father: and with him went vp all the seruants of Pharaoh, the elders of his house, and all the elders of the land of Egypt,",
        }),

        ("genesis", 50, 8) => Some(Verse {
            content: "And all the house of Ioseph, and his brethren, and his fathers house: onely their litle ones, and their flockes, and their heards, they left in the land of Goshen.",
        }),

        ("genesis", 50, 9) => Some(Verse {
            content: "And there went vp with him both charets and horsemen: and it was a very great company.",
        }),

        ("genesis", 50, 10) => Some(Verse {
            content: "And they came to the threshing floore of Atad, which is beyond Iordan, and there they mourned with a great and very sore lamentation: and he made a mourning for his father seuen dayes.",
        }),

        ("genesis", 50, 11) => Some(Verse {
            content: "And when the inhabitants of the land, the Canaanites sawe the mourning in the floore of Atad, they saide, This is a grieuous mourning to the Egyptians: wherfore the name of it was called, Abel Mizraim, which is beyond Iordan.",
        }),

        ("genesis", 50, 12) => Some(Verse {
            content: "And his sonnes did vnto him according as he commanded them.",
        }),

        ("genesis", 50, 13) => Some(Verse {
            content: "For his sonnes caried him into the land of Canaan, and buried him in the caue of the field of Machpelah, which Abraham bought with the field for a possession of a burying place, of Ephron the Hittite, before Mamre.",
        }),

        ("genesis", 50, 14) => Some(Verse {
            content: "And Ioseph returned into Egypt, he and his brethren, and all that went vp with him, to bury his father, after he had buried his father.",
        }),

        ("genesis", 50, 15) => Some(Verse {
            content: "And when Iosephs brethren saw that their father was dead, they said, Ioseph will peraduenture hate vs, and will certainely requite vs all the euill which we did vnto him.",
        }),

        ("genesis", 50, 16) => Some(Verse {
            content: "And they sent a messenger vnto Ioseph, saying, Thy father did command before he died, saying,",
        }),

        ("genesis", 50, 17) => Some(Verse {
            content: "So shall ye say vnto Ioseph, Forgiue, I pray thee now, the trespasse of thy brethren, and their sinne: for they did vnto thee euill: And now wee pray thee, forgiue the trespasse of the seruants of the God of thy father. And Ioseph wept, when they spake vnto him.",
        }),

        ("genesis", 50, 18) => Some(Verse {
            content: "And his brethren also went and fell downe before his face, and they said, Behold, we be thy seruants.",
        }),

        ("genesis", 50, 19) => Some(Verse {
            content: "And Ioseph saide vnto them, Feare not: for am I in the place of God?",
        }),

        ("genesis", 50, 20) => Some(Verse {
            content: "But as for you, yee thought euill against me, but God meant it vnto good, to bring to passe, as it is this day, to saue much people aliue.",
        }),

        ("genesis", 50, 21) => Some(Verse {
            content: "Now therefore feare yee not: I will nourish you, and your litle ones. And hee comforted them, and spake kindly vnto them.",
        }),

        ("genesis", 50, 22) => Some(Verse {
            content: "And Ioseph dwelt in Egypt, he, and his fathers house: and Ioseph liued an hundred and ten yeeres.",
        }),

        ("genesis", 50, 23) => Some(Verse {
            content: "And Ioseph sawe Ephraims children, of the third generation: the children also of Machir, the sonne of Manasseh were brought vp vpon Iosephs knees.",
        }),

        ("genesis", 50, 24) => Some(Verse {
            content: "And Ioseph saide vnto his brethren, I die: and God will surely visit you, and bring you out of this land, vnto the land which hee sware to Abraham, to Isaac, and to Iacob.",
        }),

        ("genesis", 50, 25) => Some(Verse {
            content: "And Ioseph tooke an othe of the children of Israel, saying, God will surely visite you, and ye shal carie vp my bones from hence.",
        }),

        ("genesis", 50, 26) => Some(Verse {
            content: "So Ioseph died, being an hundred and ten yeeres old: and they imbalmed him, and he was put in a coffin, in Egypt.",
        }),

        ("exodus", 1, 1) => Some(Verse {
            content: "Nowe these are the names of the children of Israel, which came into Egypt, euery man & his household, came with Iacob.",
        }),

        ("exodus", 1, 2) => Some(Verse {
            content: "Reuben, Simeon, Leui, and Iudah,",
        }),

        ("exodus", 1, 3) => Some(Verse {
            content: "Issachar, Zebulun and Beniamin,",
        }),

        ("exodus", 1, 4) => Some(Verse {
            content: "Dan, and Naphtali, Gad, and Asher.",
        }),

        ("exodus", 1, 5) => Some(Verse {
            content: "And all the soules that came out of the loynes of Iacob, were seuentie soules: for Ioseph was in Egypt already.",
        }),

        ("exodus", 1, 6) => Some(Verse {
            content: "And Ioseph died, and all his brethren, and all that generation.",
        }),

        ("exodus", 1, 7) => Some(Verse {
            content: "And the children of Israel were fruitfull, and increased aboundantly, and multiplied, and waxed exceeding mighty, and the land was filled with them.",
        }),

        ("exodus", 1, 8) => Some(Verse {
            content: "Now there arose vp a new King ouer Egypt, which knew not Ioseph.",
        }),

        ("exodus", 1, 9) => Some(Verse {
            content: "And he said vnto his people, Behold, the people of the children of Israel are moe and mightier then we.",
        }),

        ("exodus", 1, 10) => Some(Verse {
            content: "Come on, let vs deale wisely with them, lest they multiply, and it come to passe that when there falleth out any warre, they ioyne also vnto our enemies, and fight against vs, and so get them vp out of the land.",
        }),

        ("exodus", 1, 11) => Some(Verse {
            content: "Therefore they did set ouer them task-masters, to afflict them with their burdens: And they built for Pharaoh treasure-cities, Pithom and Raamses.",
        }),

        ("exodus", 1, 12) => Some(Verse {
            content: "But the more they afflicted them, the more they multiplied and grew: and they were grieued because of the children of Israel.",
        }),

        ("exodus", 1, 13) => Some(Verse {
            content: "And the Egyptians made the children of Israel to serue with rigour.",
        }),

        ("exodus", 1, 14) => Some(Verse {
            content: "And they made their liues bitter, with hard bondage, in morter and in bricke, and in all maner of seruice in the fielde: all their seruice wherein they made them serue, was with rigour.",
        }),

        ("exodus", 1, 15) => Some(Verse {
            content: "And the King of Egypt spake to the Hebrew midwiues, (of which the name of one was Shiphrah, and the name of the other Puah.)",
        }),

        ("exodus", 1, 16) => Some(Verse {
            content: "And he said, When ye do the office of a midwife to the Hebrew-women, and see them vpon the stooles, if it be a sonne, then ye shall kill him: but if it be a daughter, then shee shall liue.",
        }),

        ("exodus", 1, 17) => Some(Verse {
            content: "But the midwiues feared God, and did not as the King of Egypt commanded them, but saued the men children aliue.",
        }),

        ("exodus", 1, 18) => Some(Verse {
            content: "And the King of Egypt called for the midwiues, & said vnto them, Why haue ye done this thing, and haue saued the men children aliue?",
        }),

        ("exodus", 1, 19) => Some(Verse {
            content: "And the midwiues said vnto Pharaoh, Because the Hebrew women are not as the Egyptian women: for they are liuely, and are deliuered ere the midwiues come in vnto them.",
        }),

        ("exodus", 1, 20) => Some(Verse {
            content: "Therefore God dealt well with the midwiues: and the people multiplied and waxed very mighty.",
        }),

        ("exodus", 1, 21) => Some(Verse {
            content: "And it came to passe, because the midwiues feared God, that hee made them houses.",
        }),

        ("exodus", 1, 22) => Some(Verse {
            content: "And Pharaoh charged all his people, saying, Euery sonne that is borne, yee shall cast into the riuer, and euery daughter ye shall saue aliue.",
        }),

        ("exodus", 2, 1) => Some(Verse {
            content: "And there went a man of the house of Leui, & tooke to wife a daughter of Leui.",
        }),

        ("exodus", 2, 2) => Some(Verse {
            content: "And the woman conceiued, and bare a sonne: and when shee saw him that hee was a goodly childe, shee hid him three moneths.",
        }),

        ("exodus", 2, 3) => Some(Verse {
            content: "And when shee could not longer hide him, she tooke for him an arke of bul-rushes, and daubed it with slime, and with pitch, and put the childe therein, and shee layd it in the flags by the riuers brinke.",
        }),

        ("exodus", 2, 4) => Some(Verse {
            content: "And his sister stood afarre off, to wit what would be done to him.",
        }),

        ("exodus", 2, 5) => Some(Verse {
            content: "And the daughter of Pharaoh came downe to wash her selfe at the riuer, and her maydens walked along by the riuer side: and when shee saw the arke among the flags, she sent her maid to fetch it.",
        }),

        ("exodus", 2, 6) => Some(Verse {
            content: "And when she had opened it, she saw the childe: and beholde, the babe wept. And she had compassion on him, and said, This is one of the Hebrewes children.",
        }),

        ("exodus", 2, 7) => Some(Verse {
            content: "Then said his sister to Pharaohs daughter, Shall I goe, and call to thee a nurse of the Hebrew-women, that she may nurse the childe for thee?",
        }),

        ("exodus", 2, 8) => Some(Verse {
            content: "And Pharaohs daughter said to her, Goe: And the mayd went and called the childs mother.",
        }),

        ("exodus", 2, 9) => Some(Verse {
            content: "And Pharaohs daughter said vnto her, Take this child away, and nurse it for me, and I will giue thee thy wages. And the woman tooke the childe, and nursed it.",
        }),

        ("exodus", 2, 10) => Some(Verse {
            content: "And the childe grew, and shee brought him vnto Pharaohs daughter, and he became her sonne. And she called his name Moses: And she said, Because I drew him out of the water.",
        }),

        ("exodus", 2, 11) => Some(Verse {
            content: "And it came to passe in those dayes, when Moses was growen, that he went out vnto his brethren, and looked on their burdens, and he spied an Egyptian smiting an Hebrew, one of his brethren.",
        }),

        ("exodus", 2, 12) => Some(Verse {
            content: "And he looked this way and that way, and when he saw that there was no man, he slew the Egyptian, and hid him in the sand.",
        }),

        ("exodus", 2, 13) => Some(Verse {
            content: "And when he went out the second day, behold, two men of the Hebrewes stroue together: And hee said to him that did the wrong, Wherefore smitest thou thy fellow?",
        }),

        ("exodus", 2, 14) => Some(Verse {
            content: "And he said, Who made thee a Prince and a iudge ouer vs? intendest thou to kill me, as thou killedst the Egyptian? And Moses feared, and said, Surely this thing is knowen.",
        }),

        ("exodus", 2, 15) => Some(Verse {
            content: "Now when Pharaoh heard this thing, he sought to slay Moses. But Moses fled from the face of Pharaoh, and dwelt in the land of Midian: and he sate downe by a well.",
        }),

        ("exodus", 2, 16) => Some(Verse {
            content: "Now the Priest of Midian had seuen daughters, and they came and drew water, and filled the troughes to water their fathers flocke.",
        }),

        ("exodus", 2, 17) => Some(Verse {
            content: "And the shepheards came and droue them away: but Moses stood vp and helped them, & watred their flocke.",
        }),

        ("exodus", 2, 18) => Some(Verse {
            content: "And when they came to Reuel their father, he said, How is it that you are come so soone to day?",
        }),

        ("exodus", 2, 19) => Some(Verse {
            content: "And they said, An Egyptian deliuered vs out of the hand of the shepheards, and also drew water enough for vs, and watered the flocke.",
        }),

        ("exodus", 2, 20) => Some(Verse {
            content: "And he said vnto his daughters, And where is he? why is it that yee haue left the man? Call him, that hee may eate bread.",
        }),

        ("exodus", 2, 21) => Some(Verse {
            content: "And Moses was content to dwel with the man, and he gaue Moses Zipporah his daughter.",
        }),

        ("exodus", 2, 22) => Some(Verse {
            content: "And she bare him a sonne, and he called his name Gershom: for he said, I haue bene a stranger in a strange land.",
        }),

        ("exodus", 2, 23) => Some(Verse {
            content: "And it came to passe in processe of time, that the king of Egypt died, and the children of Israel sighed by reason of the bondage, and they cried, and their cry came vp vnto God, by reason of the bondage.",
        }),

        ("exodus", 2, 24) => Some(Verse {
            content: "And God heard their groning, and God remembred his Couenant with Abraham, with Isaac, and with Iacob.",
        }),

        ("exodus", 2, 25) => Some(Verse {
            content: "And God looked vpon the children of Israel, and God had respect vnto them.",
        }),

        ("exodus", 3, 1) => Some(Verse {
            content: "Nowe Moses kept the flocke of Iethro his father in law, the Priest of Midian: and hee led the flocke to the backeside of the desert, and came to the mountaine of God, euen to Horeb.",
        }),

        ("exodus", 3, 2) => Some(Verse {
            content: "And the Angel of the Lord appeared vnto him, in a flame of fire out of the midst of a bush, and he looked, and behold, the bush burned with fire, and the bush was not consumed.",
        }),

        ("exodus", 3, 3) => Some(Verse {
            content: "And Moses saide, I will nowe turne aside, and see this great sight, why the bush is not burnt.",
        }),

        ("exodus", 3, 4) => Some(Verse {
            content: "And when the Lord sawe that he turned aside to see, God called vnto him out of the midst of the bush, and said, Moses, Moses. And he saide, Here am I.",
        }),

        ("exodus", 3, 5) => Some(Verse {
            content: "And he said, Drawe not nigh hither: put off thy shooes from off thy feete, for the place whereon thou standest, is holy ground.",
        }),

        ("exodus", 3, 6) => Some(Verse {
            content: "Moreouer hee said, I am the God of thy father, the God of Abraham, the God of Isaac, and the God of Iacob. And Moses hid his face: for he was afraid to looke vpon God.",
        }),

        ("exodus", 3, 7) => Some(Verse {
            content: "And the Lord said, I haue surely seene the affliction of my people which are in Egypt, and haue heard their crie, by reason of their taske-masters: for I know their sorrowes,",
        }),

        ("exodus", 3, 8) => Some(Verse {
            content: "And I am come downe to deliuer them out of the hand of the Egyptians, and to bring them vp out of that land, vnto a good land and a large, vnto a lande flowing with milke and hony, vnto the place of the Canaanites, and the Hittites, and the Amorites, and the Perizzites, and the Hiuites, and the Iebusites.",
        }),

        ("exodus", 3, 9) => Some(Verse {
            content: "Now therefore behold, the crie of the children of Israel is come vnto me: and I haue also seene the oppression wherewith the Egyptians oppresse them.",
        }),

        ("exodus", 3, 10) => Some(Verse {
            content: "Come now therefore, and I will send thee vnto Pharaoh, that thou mayest bring forth my people the children of Israel out of Egypt.",
        }),

        ("exodus", 3, 11) => Some(Verse {
            content: "And Moses saide vnto God, Who am I, that I should goe vnto Pharaoh, and that I should bring forth the children of Israel out of Egypt?",
        }),

        ("exodus", 3, 12) => Some(Verse {
            content: "And he said, Certainely I will be with thee, and this shall be a token vnto thee, that I haue sent thee: When thou hast brought foorth the people out of Egypt, ye shall serue God vpon this mountaine.",
        }),

        ("exodus", 3, 13) => Some(Verse {
            content: "And Moses saide vnto God, Behold, when I come vnto the children of Israel, and shall say vnto them, The God of your fathers hath sent me vnto you; and they shall say to me, What is his name? what shall I say vnto them?",
        }),

        ("exodus", 3, 14) => Some(Verse {
            content: "And God saide vnto Moses, I AM THAT I AM: And he said, Thus shalt thou say vnto the children of Israel, I AM hath sent me vnto you.",
        }),

        ("exodus", 3, 15) => Some(Verse {
            content: "And God said moreouer vnto Moses, Thus shalt thou say vnto the children of Israel; The Lord God of your fathers, the God of Abraham, the God of Isaac, and the God of Iacob hath sent me vnto you: this is my name for euer, and this is my memoriall vnto all generations.",
        }),

        ("exodus", 3, 16) => Some(Verse {
            content: "Goe and gather the Elders of Israel together, and say vnto them, The Lord God of your fathers, the God of Abraham, of Isaac, and of Iacob appeared vnto me, saying, I haue surely visited you, and seene that which is done to you in Egypt.",
        }),

        ("exodus", 3, 17) => Some(Verse {
            content: "And I haue said, I will bring you vp out of the affliction of Egypt, vnto the land of the Canaanites, and the Hittites, and the Amorites, and the Perizzites, and the Hiuites, and the Iebusites, vnto a land flowing with milke and hony.",
        }),

        ("exodus", 3, 18) => Some(Verse {
            content: "And they shall hearken to thy voyce: and thou shalt come, thou and the Elders of Israel vnto the King of Egypt, and you shall say vnto him, The Lord God of the Hebrewes hath met with vs: and now let vs goe, (wee beseech thee) three dayes iourney into the wildernes, that we may sacrifice to the Lord our God.",
        }),

        ("exodus", 3, 19) => Some(Verse {
            content: "And I am sure that the King of Egypt will not let you goe, no not by a mightie hand.",
        }),

        ("exodus", 3, 20) => Some(Verse {
            content: "And I will stretch out my hand, and smite Egypt with all my wonders which I will doe in the midst thereof: and after that he will let you goe.",
        }),

        ("exodus", 3, 21) => Some(Verse {
            content: "And I will giue this people fauour in the sight of the Egyptians, and it shall come to passe that when ye goe, ye shall not goe empty:",
        }),

        ("exodus", 3, 22) => Some(Verse {
            content: "But euery woman shal borrow of her neighbour, and of her that soiourneth in her house, iewels of siluer, and iewels of gold, and rayment: and ye shall put them vpon your sonnes and vpon your daughters, and yee shall spoile the Egyptians.",
        }),

        ("exodus", 4, 1) => Some(Verse {
            content: "And Moses answered, and said, But behold, they will not beleeue mee, nor hearken vnto my voice: for they will say, The Lord hath not appeared vnto thee.",
        }),

        ("exodus", 4, 2) => Some(Verse {
            content: "And the Lord said vnto him, What is that in thine hand? and hee said, A rod.",
        }),

        ("exodus", 4, 3) => Some(Verse {
            content: "And he said, Cast it on the ground: And he cast it on the ground, and it became a serpent: and Moses fled from before it.",
        }),

        ("exodus", 4, 4) => Some(Verse {
            content: "And the Lord said vnto Moses, Put forth thine hand, and take it by the taile: And he put foorth his hand, and caught it, and it became a rod in his hand:",
        }),

        ("exodus", 4, 5) => Some(Verse {
            content: "That they may beleeue that the Lord God of their fathers, the God of Abraham, the God of Isaac, and the God of Iacob hath appeared vnto thee.",
        }),

        ("exodus", 4, 6) => Some(Verse {
            content: "And the Lord said furthermore vnto him, Put now thine hand into thy bosome. And he put his hand into his bosome: and when hee tooke it out, behold, his hand was leprous as snowe.",
        }),

        ("exodus", 4, 7) => Some(Verse {
            content: "And he said, Put thine hand into thy bosome againe. And hee put his hand into his bosome againe, and plucked it out of his bosome, and behold, it was turned againe as his other flesh.",
        }),

        ("exodus", 4, 8) => Some(Verse {
            content: "And it shall come to passe, if they wil not beleeue thee, neither hearken to the voice of the first signe, that they will beleeue the voice of the latter signe.",
        }),

        ("exodus", 4, 9) => Some(Verse {
            content: "And it shall come to passe, if they will not beleeue also these two signes, neither hearken vnto thy voice, that thou shalt take of the water of the riuer, and powre it vpon the drie land: and the water which thou takest out of the riuer, shall become blood vpon the drie land.",
        }),

        ("exodus", 4, 10) => Some(Verse {
            content: "And Moses saide vnto the Lord, O my lord, I am not eloquent, neither heretofore, nor since thou hast spoken vnto thy seruant: but I am slow of speach, and of a slow tongue.",
        }),

        ("exodus", 4, 11) => Some(Verse {
            content: "And the Lord said vnto him, Who hath made mans mouth? or who maketh the dumbe or deafe, or the seeing, or þe blind? haue not I the Lord?",
        }),

        ("exodus", 4, 12) => Some(Verse {
            content: "Now therefore goe, and I will be with thy mouth, and teach thee what thou shalt say.",
        }),

        ("exodus", 4, 13) => Some(Verse {
            content: "And he said, O my Lord, send, I pray thee, by the hand of him whom thou wilt send.",
        }),

        ("exodus", 4, 14) => Some(Verse {
            content: "And the anger of the Lord was kindled against Moses, and hee said, Is not Aaron the Leuite thy brother? I know that he can speake well. And also behold, he commeth foorth to meet thee: and when he seeth thee, hee will be glad in his heart.",
        }),

        ("exodus", 4, 15) => Some(Verse {
            content: "And thou shalt speake vnto him, and put words in his mouth, and I wil be with thy mouth, & with his mouth, and will teach you what ye shall doe.",
        }),

        ("exodus", 4, 16) => Some(Verse {
            content: "And he shal be thy spokesman vnto the people: and he shall be, euen hee shall be to thee in stead of a mouth, and thou shalt be to him in stead of God.",
        }),

        ("exodus", 4, 17) => Some(Verse {
            content: "And thou shalt take this rod in thine hand, wherewith thou shalt doe signes.",
        }),

        ("exodus", 4, 18) => Some(Verse {
            content: "And Moses went and returned to Iethro his father in law, and said vnto him, Let me goe, I pray thee, and returne vnto my brethren, which are in Egypt, and see whether they bee yet aliue. And Iethro said to Moses, Goe in peace.",
        }),

        ("exodus", 4, 19) => Some(Verse {
            content: "And the Lord said vnto Moses in Midian, Goe, returne into Egypt: for all the men are dead which sought thy life.",
        }),

        ("exodus", 4, 20) => Some(Verse {
            content: "And Moses tooke his wife, and his sonnes, and set them vpon an asse, and he returned to the land of Egypt. And Moses tooke the rod of God in his hand.",
        }),

        ("exodus", 4, 21) => Some(Verse {
            content: "And the Lord said vnto Moses, When thou goest to returne into Egypt, see that thou doe all those wonders before Pharaoh, which I haue put in thine hand: but I wil harden his heart, that hee shall not let the people goe.",
        }),

        ("exodus", 4, 22) => Some(Verse {
            content: "And thou shalt say vnto Pharaoh, Thus saith the Lord, Israel is my sonne, euen my first borne.",
        }),

        ("exodus", 4, 23) => Some(Verse {
            content: "And I say vnto thee, let my sonne goe, that he may serue mee: and if thou refuse to let him goe, behold, I will slay thy sonne, euen thy first borne.",
        }),

        ("exodus", 4, 24) => Some(Verse {
            content: "And it came to passe by the way in the Inne, that the Lord met him, and sought to kill him.",
        }),

        ("exodus", 4, 25) => Some(Verse {
            content: "Then Zipporah tooke a sharpe stone, and cut off the foreskinne of her sonne, and cast it at his feete, and said, Surely a bloody husband art thou to mee.",
        }),

        ("exodus", 4, 26) => Some(Verse {
            content: "So he let him goe: then she said, A bloody husband thou art, because of the Circumcision.",
        }),

        ("exodus", 4, 27) => Some(Verse {
            content: "And the Lord said to Aaron, Goe into the wildernesse to meete Moses. And hee went and met him in the mount of God, and kissed him.",
        }),

        ("exodus", 4, 28) => Some(Verse {
            content: "And Moses tolde Aaron all the wordes of the Lord, who had sent him, and all the signes which hee had commanded him.",
        }),

        ("exodus", 4, 29) => Some(Verse {
            content: "And Moses and Aaron went, and gathered together all the elders of the children of Israel.",
        }),

        ("exodus", 4, 30) => Some(Verse {
            content: "And Aaron spake all the wordes which the Lord had spoken vnto Moses, and did the signes in the sight of the people.",
        }),

        ("exodus", 4, 31) => Some(Verse {
            content: "And the people beleeued: And when they heard that the Lord had visited the children of Israel, and that he had looked vpon their affliction, then they bowed their heads and worshipped.",
        }),

        ("exodus", 5, 1) => Some(Verse {
            content: "And afterward Moses and Aaron went in, and tolde Pharaoh, Thus saith the Lord God of Israel, Let my people goe, that they may holde a feast vnto mee in the wildernesse.",
        }),

        ("exodus", 5, 2) => Some(Verse {
            content: "And Pharaoh said, Who is the Lord, that I should obey his voyce to let Israel goe? I know not the Lord, neither will I let Israel goe.",
        }),

        ("exodus", 5, 3) => Some(Verse {
            content: "And they said, The God of the Hebrewes hath met with vs: let vs goe, we pray thee, three dayes iourney into the desert, and sacrifice vnto the Lord our God, lest hee fall vpon vs with pestilence, or with the sword.",
        }),

        ("exodus", 5, 4) => Some(Verse {
            content: "And the King of Egypt said vnto them, Wherfore doe ye, Moses and Aaron, let the people from their workes? get you vnto your burdens.",
        }),

        ("exodus", 5, 5) => Some(Verse {
            content: "And Pharaoh said, Behold, the people of the land now are many, & you make them rest from their burdens.",
        }),

        ("exodus", 5, 6) => Some(Verse {
            content: "And Pharaoh commanded the same day the taske-masters of the people, and their officers, saying;",
        }),

        ("exodus", 5, 7) => Some(Verse {
            content: "Yee shall no more giue the people straw to make bricke, as heretofore: let them goe and gather straw for themselues.",
        }),

        ("exodus", 5, 8) => Some(Verse {
            content: "And the tale of the brickes which they did make heretofore, you shall lay vpon them: you shall not diminish ought thereof: for they be idle; therefore they cry, saying, Let us goe and sacrifice to our God.",
        }),

        ("exodus", 5, 9) => Some(Verse {
            content: "Let there more worke be layde vpon the men, that they may labour therein, and let them not regard vaine wordes.",
        }),

        ("exodus", 5, 10) => Some(Verse {
            content: "And the taske-masters of the people went out, and their officers, and they spake to the people, saying, Thus saith Pharaoh, I will not giue you straw.",
        }),

        ("exodus", 5, 11) => Some(Verse {
            content: "Goe ye, get you straw where you can find it: yet not ought of your worke shall be diminished.",
        }),

        ("exodus", 5, 12) => Some(Verse {
            content: "So the people were scattered abroad throughout al the land of Egypt, to gather stubble in stead of straw.",
        }),

        ("exodus", 5, 13) => Some(Verse {
            content: "And the taske-masters hasted them, saying; Fulfill your workes, your dayly taskes, as when there was straw.",
        }),

        ("exodus", 5, 14) => Some(Verse {
            content: "And the officers of the children of Israel, which Pharaohs task-masters had set ouer them, were beaten, and demanded, Wherefore haue ye not fulfilled your taske, in making bricke, both yesterday and to day, as heretofore?",
        }),

        ("exodus", 5, 15) => Some(Verse {
            content: "Then the officers of the children of Israel came and cryed vnto Pharaoh, saying, Wherefore dealest thou thus with thy seruants?",
        }),

        ("exodus", 5, 16) => Some(Verse {
            content: "There is no straw giuen vnto thy seruants, and they say to vs, Make bricke: and beholde, thy seruants are beaten; but the fault is in thine owne people.",
        }),

        ("exodus", 5, 17) => Some(Verse {
            content: "But he said, Ye are idle, ye are idle: therefore ye say, Let vs goe and doe sacrifice to the Lord.",
        }),

        ("exodus", 5, 18) => Some(Verse {
            content: "Goe therefore now and worke: for there shall no straw bee giuen you, yet shall ye deliuer the tale of brickes.",
        }),

        ("exodus", 5, 19) => Some(Verse {
            content: "And the officers of the children of Israel did see that they were in euill case, after it was said, Yee shall not minish ought from your brickes of your dayly taske.",
        }),

        ("exodus", 5, 20) => Some(Verse {
            content: "And they met Moses and Aaron, who stood in the way, as they came foorth from Pharaoh.",
        }),

        ("exodus", 5, 21) => Some(Verse {
            content: "And they said vnto them; The Lord looke vpon you, and iudge, because you haue made our sauour to be abhorred in the eyes of Pharaoh, and in the eyes of his seruants, to put a sword in their hand to slay vs.",
        }),

        ("exodus", 5, 22) => Some(Verse {
            content: "And Moses returned vnto the Lord, and said, Lord, Wherefore hast thou so euill intreated this people? Why is it that thou hast sent me?",
        }),

        ("exodus", 5, 23) => Some(Verse {
            content: "For since I came to Pharaoh to speake in thy Name, he hath done euill to this people, neither hast thou deliuered thy people at all.",
        }),

        ("exodus", 6, 1) => Some(Verse {
            content: "Then the Lord said vnto Moses, Now shalt thou see what I will doe to Pharaoh: for with a strong hand shall hee let them goe, and with a strong hand shall he driue them out of his land.",
        }),

        ("exodus", 6, 2) => Some(Verse {
            content: "And God spake vnto Moses, and said vnto him, I am the Lord.",
        }),

        ("exodus", 6, 3) => Some(Verse {
            content: "And I appeared vnto Abraham, vnto Isaac, and vnto Iacob, by the Name of God Almighty, but by my name IEHOVAH was I not knowen to them.",
        }),

        ("exodus", 6, 4) => Some(Verse {
            content: "And I haue also established my Couenant with them, to giue them the land of Canaan, the land of their pilgrimage, wherein they were strangers.",
        }),

        ("exodus", 6, 5) => Some(Verse {
            content: "And I haue also heard the groning of the children of Israel, whom the Egyptians keepe in bondage: and I haue remembred my Couenant.",
        }),

        ("exodus", 6, 6) => Some(Verse {
            content: "Wherefore say vnto the children of Israel, I am the Lord, and I will bring you out from vnder the burdens of the Egyptians, and I will rid you out of their bondage: and I will redeeme you with a stretched out arme, and with great iudgements.",
        }),

        ("exodus", 6, 7) => Some(Verse {
            content: "And I will take you to mee for a people, and I will be to you a God: and ye shall know that I am the Lord your God, which bringeth you out from vnder the burdens of the Egyptians.",
        }),

        ("exodus", 6, 8) => Some(Verse {
            content: "And I will bring you in vnto the lande concerning the which I did sweare to giue it, to Abraham, to Isaac, and to Iacob, and I will giue it you for an heritage, I am the Lord.",
        }),

        ("exodus", 6, 9) => Some(Verse {
            content: "And Moses spake so vnto the children of Israel: but they hearkened not vnto Moses, for anguish of spirit, and for cruell bondage.",
        }),

        ("exodus", 6, 10) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 6, 11) => Some(Verse {
            content: "Goe in, speake vnto Pharaoh King of Egypt, that he let the children of Israel goe out of his land.",
        }),

        ("exodus", 6, 12) => Some(Verse {
            content: "And Moses spake before the Lord, saying, Behold, the children of Israel haue not hearkened vnto me: how then shal Pharaoh heare me, who am of vncircumcised lips?",
        }),

        ("exodus", 6, 13) => Some(Verse {
            content: "And the Lord spake vnto Moses and vnto Aaron, & gaue them a charge vnto the children of Israel, and vnto Pharaoh King of Egypt, to bring the children of Israel out of the land of Egypt.",
        }),

        ("exodus", 6, 14) => Some(Verse {
            content: "These be the heads of their fathers houses: The sonnes of Reuben the first borne of Israel, Hanoch, and Pallu, Hezron, and Carmi: these be the families of Reuben.",
        }),

        ("exodus", 6, 15) => Some(Verse {
            content: "And the sonnes of Simeon: Iemuel, and Iamin, and Ohad and Iachin, and Zohar, and Shaul the sonne of a Canaanitish woman: these are the families of Simeon.",
        }),

        ("exodus", 6, 16) => Some(Verse {
            content: "And these are the names of the sonnes of Leui, according to their generations: Gershon and Kohath and Merari: and the yeeres of the life of Leui, were an hundred, thirtie and seuen yeeres.",
        }),

        ("exodus", 6, 17) => Some(Verse {
            content: "The sonnes of Gershon: Libni and Shimi, according to their families.",
        }),

        ("exodus", 6, 18) => Some(Verse {
            content: "And the sonnes of Kohath: Amram, and Izhar, and Hebron, and Uzziel. And the yeeres of the life of Kohath, were an hundred thirtie and three yeeres.",
        }),

        ("exodus", 6, 19) => Some(Verse {
            content: "And the sonnes of Merari: Mahali and Mushi: these are the families of Leui, according to their generations.",
        }),

        ("exodus", 6, 20) => Some(Verse {
            content: "And Amram tooke him Iochebed his fathers sister to wife, and shee bare him Aaron and Moses: and the yeeres of the life of Amram were an hundred, and thirtie and seuen yeeres.",
        }),

        ("exodus", 6, 21) => Some(Verse {
            content: "And the sonnes of Izhar: Korah and Nepheg, and Zichri.",
        }),

        ("exodus", 6, 22) => Some(Verse {
            content: "And the sonnes of Uzziel: Mishael, and Elzaphan, and Zithri.",
        }),

        ("exodus", 6, 23) => Some(Verse {
            content: "And Aaron tooke him Elisheba daughter of Amminadab sister of Naashon to wife, and she bare him Nadab and Abihu, Eleazar and Ithamar.",
        }),

        ("exodus", 6, 24) => Some(Verse {
            content: "And the sonnes of Korah, Assir, and Elkanah, and Abiasaph: these are the families of the Korhites.",
        }),

        ("exodus", 6, 25) => Some(Verse {
            content: "And Eleazar Aarons sonne tooke him one of the daughters of Putiel to wife, and she bare him Phinehas: these are the heads of the fathers of the Leuites, according to their families.",
        }),

        ("exodus", 6, 26) => Some(Verse {
            content: "These are that Aaron and Moses, to whom the Lord said, Bring out the children of Israel from the land of Egypt, according to their armies.",
        }),

        ("exodus", 6, 27) => Some(Verse {
            content: "These are they which spake to Pharaoh king of Egypt, to bring out the children of Israel from Egypt: These are that Moses and Aaron.",
        }),

        ("exodus", 6, 28) => Some(Verse {
            content: "And it came to passe on the day when the Lord spake vnto Moses in the land of Egypt,",
        }),

        ("exodus", 6, 29) => Some(Verse {
            content: "That the Lord spake vnto Moses, saying, I am the Lord: speake thou vnto Pharaoh king of Egypt, all that I say vnto thee.",
        }),

        ("exodus", 6, 30) => Some(Verse {
            content: "And Moses said before the Lord, Behold, I am of vncircumcised lips, and how shall Pharaoh hearken vnto mee?",
        }),

        ("exodus", 7, 1) => Some(Verse {
            content: "And the Lord said vnto Moses, See, I haue made thee a god to Pharaoh, and Aaron thy brother shalbe thy prophet.",
        }),

        ("exodus", 7, 2) => Some(Verse {
            content: "Thou shalt speake all that I command thee, and Aaron thy brother shall speake vnto Pharaoh, that he send the children of Israel out of his land.",
        }),

        ("exodus", 7, 3) => Some(Verse {
            content: "And I will harden Pharaohs heart, and multiplie my signes and my wonders in the land of Egypt.",
        }),

        ("exodus", 7, 4) => Some(Verse {
            content: "But Pharaoh shall not hearken vnto you, that I may lay my hand vpon Egypt, and bring forth mine armies, and my people the children of Israel, out of the land of Egypt, by great iudgments.",
        }),

        ("exodus", 7, 5) => Some(Verse {
            content: "And the Egyptians shall knowe that I am the Lord, when I stretch forth mine hand vpon Egypt, and bring out the children of Israel from among them.",
        }),

        ("exodus", 7, 6) => Some(Verse {
            content: "And Moses and Aaron did as the Lord commanded them, so did they.",
        }),

        ("exodus", 7, 7) => Some(Verse {
            content: "And Moses was fourescore yeres olde, and Aaron fourescore and three yeeres old, when they spake vnto Pharaoh.",
        }),

        ("exodus", 7, 8) => Some(Verse {
            content: "And the Lord spake vnto Moses, and vnto Aaron, saying:",
        }),

        ("exodus", 7, 9) => Some(Verse {
            content: "When Pharaoh shall speake vnto you, saying, Shew a miracle for you: then thou shalt say vnto Aaron, Take thy rod and cast it before Pharaoh, and it shall become a serpent.",
        }),

        ("exodus", 7, 10) => Some(Verse {
            content: "And Moses and Aaron went in vnto Pharaoh, and they did so as the Lord had commanded: and Aaron cast downe his rod before Pharaoh, and before his seruants, and it became a serpent.",
        }),

        ("exodus", 7, 11) => Some(Verse {
            content: "Then Pharaoh also called the wise men and the sorcerers; now the Magicians of Egypt, they also did in like maner with their enchantments.",
        }),

        ("exodus", 7, 12) => Some(Verse {
            content: "For they cast downe euery man his rod, and they became serpents: but Aarons rod swallowed vp their rods.",
        }),

        ("exodus", 7, 13) => Some(Verse {
            content: "And hee hardened Pharaohs heart, that hee hearkened not vnto them, as the Lord had said.",
        }),

        ("exodus", 7, 14) => Some(Verse {
            content: "And the Lord saide vnto Moses, Pharaohs heart is hardened: he refuseth to let the people goe.",
        }),

        ("exodus", 7, 15) => Some(Verse {
            content: "Get thee vnto Pharaoh in the morning, loe, he goeth out vnto the water, and thou shalt stand by the riuers brinke, against hee come: and the rod which was turned to a serpent, shalt thou take in thine hand.",
        }),

        ("exodus", 7, 16) => Some(Verse {
            content: "And thou shalt say vnto him, The Lord God of the Hebrewes hath sent me vnto thee, saying; Let my people goe, that they may serue mee in the wildernesse: and beholde, hitherto thou wouldest not heare.",
        }),

        ("exodus", 7, 17) => Some(Verse {
            content: "Thus saith the Lord, In this thou shalt know that I am the Lord: behold, I will smite with the rod that is in my hand, vpon the waters which are in the riuer, and they shalbe turned to blood.",
        }),

        ("exodus", 7, 18) => Some(Verse {
            content: "And the fish that is in the riuer shall die, and the riuer shall stincke, and the Egyptians shall loathe to drinke of the water of the riuer.",
        }),

        ("exodus", 7, 19) => Some(Verse {
            content: "And the Lord spake vnto Moses, Say vnto Aaron, Take thy rod, & stretch out thine hand vpon the waters of Egypt, vpon their streames, vpon their riuers, and vpon their ponds, and vpon all their pooles of water, that they may become blood, and that there may be blood throughout all the land of Egypt, both in vessels of wood, and in vessels of stone.",
        }),

        ("exodus", 7, 20) => Some(Verse {
            content: "And Moses and Aaron did so, as the Lord commanded: and he lift vp the rod and smote the waters that were in the riuer, in the sight of Pharaoh, and in the sight of his seruants: and all the waters that were in the riuer, were turned to blood.",
        }),

        ("exodus", 7, 21) => Some(Verse {
            content: "And the fish that was in the riuer died: and the riuer stunke, and the Egyptians could not drinke of the water of the riuer: and there was blood throughout all the land Egypt.",
        }),

        ("exodus", 7, 22) => Some(Verse {
            content: "And the Magicians of Egypt did so, with their enchantments: and Pharaohs heart was hardened, neither did he hearken vnto them, as the Lord had said.",
        }),

        ("exodus", 7, 23) => Some(Verse {
            content: "And Pharaoh turned and went into his house, neither did hee set his heart to this also.",
        }),

        ("exodus", 7, 24) => Some(Verse {
            content: "And all the Egyptians digged round about the riuer for water to drinke: for they could not drinke of the water of the riuer.",
        }),

        ("exodus", 7, 25) => Some(Verse {
            content: "And seuen dayes were fulfilled after that the Lord had smitten the riuer.",
        }),

        ("exodus", 8, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses, Goe vnto Pharaoh, and say vnto him; Thus sayeth the Lord, Let my people goe, that they may serue me.",
        }),

        ("exodus", 8, 2) => Some(Verse {
            content: "And if thou refuse to let them goe, beholde, I will smite all thy borders with frogges.",
        }),

        ("exodus", 8, 3) => Some(Verse {
            content: "And the riuer shall bring foorth frogges abundantly, which shall goe vp and come into thine house, and into thy bed-chamber, and vpon thy bed, and into the house of thy seruants, and vpon thy people, and into thine ouens, and into thy kneading troughes.",
        }),

        ("exodus", 8, 4) => Some(Verse {
            content: "And the frogges shall come vp both on thee, and vpon thy people, and vpon all thy seruants.",
        }),

        ("exodus", 8, 5) => Some(Verse {
            content: "And the Lord spake vnto Moses; Say vnto Aaron, Stretch foorth thine hand with thy rodde ouer the streames, ouer the riuers, and ouer the ponds, and cause frogges to come vp vpon the land of Egypt.",
        }),

        ("exodus", 8, 6) => Some(Verse {
            content: "And Aaron stretched out his hand ouer the waters of Egypt, and the frogges came vp, and couered the land of Egypt.",
        }),

        ("exodus", 8, 7) => Some(Verse {
            content: "And the Magicians did so with their inchantments, and brought vp frogges vpon the land of Egypt.",
        }),

        ("exodus", 8, 8) => Some(Verse {
            content: "Then Pharaoh called for Moses, and Aaron, and said, Intreat the Lord, that hee may take away the frogges from me, and from my people: and I will let the people goe, that they may doe sacrifice vnto the Lord.",
        }),

        ("exodus", 8, 9) => Some(Verse {
            content: "And Moses saide vnto Pharaoh, Glory ouer mee: when shall I entreat for thee, and for thy seruants, and for thy people, to destroy the frogges from thee, and thy houses, that they may remaine in the riuer onely?",
        }),

        ("exodus", 8, 10) => Some(Verse {
            content: "And he said, To morrow. And hee said, Bee it according to thy word: That thou mayest know that there is none like vnto the Lord our God.",
        }),

        ("exodus", 8, 11) => Some(Verse {
            content: "And the frogs shall depart from thee, and from thy houses, and from thy seruants, and from thy people; they shall remaine in the riuer onely.",
        }),

        ("exodus", 8, 12) => Some(Verse {
            content: "And Moses and Aaron went out from Pharaoh, and Moses cried vnto the Lord because of the frogs which he had brought against Pharaoh.",
        }),

        ("exodus", 8, 13) => Some(Verse {
            content: "And the Lord did according to the word of Moses: and the frogges died out of the houses, out of the villages, and out of the fields.",
        }),

        ("exodus", 8, 14) => Some(Verse {
            content: "And they gathered them together vpon heapes, and the land stanke.",
        }),

        ("exodus", 8, 15) => Some(Verse {
            content: "But when Pharaoh saw that there was respit, he hardned his heart, and hearkened not vnto them, as the Lord had said.",
        }),

        ("exodus", 8, 16) => Some(Verse {
            content: "And the Lord saide vnto Moses, Say vnto Aaron, Stretch out thy rod, and smite the dust of the land, that it may become lice, thorowout all the land of Egypt.",
        }),

        ("exodus", 8, 17) => Some(Verse {
            content: "And they did so: for Aaron stretched out his hand with his rod, and smote the dust of the earth, and it became lice, in man and in beast: all the dust of the land became lice throughout all the land of Egypt.",
        }),

        ("exodus", 8, 18) => Some(Verse {
            content: "And the Magicians did so with their enchantments to bring foorth lice, but they could not: so there were lice vpon man and vpon beast.",
        }),

        ("exodus", 8, 19) => Some(Verse {
            content: "Then the Magicians said vnto Pharaoh; This is the singer of God. And Pharaohs heart was hardned, and he hearkened not vnto them, as the Lord had said.",
        }),

        ("exodus", 8, 20) => Some(Verse {
            content: "And the Lord saide vnto Moses, Rise vp early in the morning, and stand before Pharaoh: loe, he commeth foorth to the water, and say vnto him; Thus saith the Lord, Let my people goe, that they may serue me.",
        }),

        ("exodus", 8, 21) => Some(Verse {
            content: "Els, if thou wilt not let my people goe, beholde, I will send swarmes of flies vpon thee, and vpon thy seruants, and vpon thy people, and into thy houses: and the houses of the Egyptians shall bee full of swarmes of flies, and also the ground whereon they are.",
        }),

        ("exodus", 8, 22) => Some(Verse {
            content: "And I will seuer in that day the lande of Goshen in which my people dwell, that no swarmes of flies shall be there, to the end thou maiest know that I am the Lord in the midst of the earth.",
        }),

        ("exodus", 8, 23) => Some(Verse {
            content: "And I will put a diuision betweene my people and thy people: to morrow shall this signe be.",
        }),

        ("exodus", 8, 24) => Some(Verse {
            content: "And the Lord did so: and there came a grieuous swarme of flies into the house of Pharaoh, and into his seruants houses, and into all the lande of Egypt: the land was corrupted by reason of the swarme of flies.",
        }),

        ("exodus", 8, 25) => Some(Verse {
            content: "And Pharaoh called for Moses and for Aaron, and said, Goe yee, sacrifice to your God in the land.",
        }),

        ("exodus", 8, 26) => Some(Verse {
            content: "And Moses said, It is not meete so to doe; for we shal sacrifice the abomination of the Egyptians, to the Lord our God: Loe, shall we sacrifice the abomination of the Egyptians before their eyes, and will they not stone vs?",
        }),

        ("exodus", 8, 27) => Some(Verse {
            content: "We will goe three dayes iourney into the wildernesse, and sacrifice to the Lord our God, as he shall command vs.",
        }),

        ("exodus", 8, 28) => Some(Verse {
            content: "And Pharaoh said, I wil let you goe that ye may sacrifice to the Lord your God, in the wildernes: onely you shall not goe very farre away: intreate for me.",
        }),

        ("exodus", 8, 29) => Some(Verse {
            content: "And Moses said, Behold, I goe out from thee, and I will intreate the Lord that the swarmes of flies may depart from Pharaoh, from his seruants, and from his people to morrow: but let not Pharaoh deale deceitfully any more, in not letting the people goe to sacrifice to the Lord.",
        }),

        ("exodus", 8, 30) => Some(Verse {
            content: "And Moses went out from Pharaoh, and intreated the Lord:",
        }),

        ("exodus", 8, 31) => Some(Verse {
            content: "And the Lord did according to the word of Moses: and he remooued the swarmes of flies from Pharaoh, from his seruants, and from his people: there remained not one.",
        }),

        ("exodus", 8, 32) => Some(Verse {
            content: "And Pharaoh hardened his heart at this time also, neither would hee let the people goe.",
        }),

        ("exodus", 9, 1) => Some(Verse {
            content: "Then the Lord said vnto Moses, Goe in vnto Pharaoh, and tell him, Thus saith the Lord God of the Hebrewes, Let my people goe, that they may serue me.",
        }),

        ("exodus", 9, 2) => Some(Verse {
            content: "For if thou refuse to let them goe, and wilt hold them still,",
        }),

        ("exodus", 9, 3) => Some(Verse {
            content: "Behold, the hand of the Lord is vpon thy cattell which is in the field, vpon the horses, vpon the asses, vpon the camels, vpon the oxen, and vpon the sheepe: there shall be a very grieuous murraine.",
        }),

        ("exodus", 9, 4) => Some(Verse {
            content: "And the Lord shall seuer betweene the cattell of Israel, and the cattell of Egypt, and there shall nothing die of all that is the childrens of Israel.",
        }),

        ("exodus", 9, 5) => Some(Verse {
            content: "And the Lord appointed a set time, saying, To morrow the Lord shall doe this thing in the land.",
        }),

        ("exodus", 9, 6) => Some(Verse {
            content: "And the Lord did that thing on the morrow; and all the cattell of Egypt died, but of the cattell of the children of Israel died not one.",
        }),

        ("exodus", 9, 7) => Some(Verse {
            content: "And Pharaoh sent, and beholde, there was not one of the cattell of the Israelites dead. And the heart of Pharaoh was hardened, and he did not let the people goe.",
        }),

        ("exodus", 9, 8) => Some(Verse {
            content: "And the Lord saide vnto Moses, and vnto Aaron, Take to you handfuls of ashes of the fornace, and let Moses sprinkle it towards the heauen, in the sight of Pharaoh:",
        }),

        ("exodus", 9, 9) => Some(Verse {
            content: "And it shall become small dust in all the land of Egypt, and shall bee a boyle breaking forth with blaines, vpon man and vpon beast, throughout all the land of Egypt.",
        }),

        ("exodus", 9, 10) => Some(Verse {
            content: "And they tooke ashes of the fornace, and stood before Pharaoh, and Moses sprinkled it vp toward heauen: and it became a boile breaking forth with blaines, vpon man and vpon beast.",
        }),

        ("exodus", 9, 11) => Some(Verse {
            content: "And the Magicians could not stand before Moses, because of the boiles: for the boile was vpon the magicians, and vpon all the Egyptians.",
        }),

        ("exodus", 9, 12) => Some(Verse {
            content: "And the Lord hardened the heart of Pharaoh, and hee hearkened not vnto them, as the Lord had spoken vnto Moses.",
        }),

        ("exodus", 9, 13) => Some(Verse {
            content: "And the Lord saide vnto Moses, Rise vp earely in the morning, and stand before Pharaoh, and say vnto him, Thus saith the Lord God of the Hebrewes, Let my people goe, that they may serue me.",
        }),

        ("exodus", 9, 14) => Some(Verse {
            content: "For I will at this time send all my plagues vpon thine heart, and vpon thy seruants, and vpon thy people: that thou mayest knowe that there is none like me in all the earth.",
        }),

        ("exodus", 9, 15) => Some(Verse {
            content: "For now I will stretch out my hand, that I may smite thee and thy people, with pestilence, and thou shalt be cut off from the earth.",
        }),

        ("exodus", 9, 16) => Some(Verse {
            content: "And in very deede, for this cause haue I raised thee vp, for to shewe in thee my power, and that my name may be declared throughout all the earth.",
        }),

        ("exodus", 9, 17) => Some(Verse {
            content: "As yet exaltest thou thy selfe against my people, that thou wilt not let them goe?",
        }),

        ("exodus", 9, 18) => Some(Verse {
            content: "Behold, to morrow about this time, I wil cause it to raine a very grieuous haile, such as hath not bene in Egypt, since the foundation thereof euen vntill now.",
        }),

        ("exodus", 9, 19) => Some(Verse {
            content: "Send therefore now, and gather thy cattell, and all that thou hast in the field: for vpon euery man and beast which shal be found in the field, and shal not bee brought home, the haile shall come downe vpon them, and they shall die.",
        }),

        ("exodus", 9, 20) => Some(Verse {
            content: "Hee that feared the word of the Lord amongst the seruants of Pharaoh, made his seruants and his cattell flee into the houses.",
        }),

        ("exodus", 9, 21) => Some(Verse {
            content: "And he that regarded not the word of the Lord, left his seruants and his cattell in the field.",
        }),

        ("exodus", 9, 22) => Some(Verse {
            content: "And the Lord saide vnto Moses, Stretch forth thine hand toward heauen, that there may be haile in all the land of Egypt, vpon man and vpon beast, and vpon euery herbe of the field, thorowout the land of Egypt.",
        }),

        ("exodus", 9, 23) => Some(Verse {
            content: "And Moses stretched foorth his rod toward heauen, and the Lord sent thunder and haile, and the fire ranne along vpon the ground, and the Lord rained haile vpon the land of Egypt.",
        }),

        ("exodus", 9, 24) => Some(Verse {
            content: "So there was haile, and fire mingled with the haile, very grieuous, such as there was none like it in all the land of Egypt, since it became a nation.",
        }),

        ("exodus", 9, 25) => Some(Verse {
            content: "And the haile smote throughout all the land of Egypt, all that was in the field, both man and beast: and the haile smote euery herbe of the fielde, and brake euery tree of the field.",
        }),

        ("exodus", 9, 26) => Some(Verse {
            content: "Onely in the land of Goshen where the children of Israel were, was there no haile.",
        }),

        ("exodus", 9, 27) => Some(Verse {
            content: "And Pharaoh sent, and called for Moses and Aaron, and said vnto them, I haue sinned this time: the Lord is righteous, and I and my people are wicked.",
        }),

        ("exodus", 9, 28) => Some(Verse {
            content: "Entreat the Lord, (for it is enough) that there be no more mighty thunderings and haile, and I will let you goe, and ye shall stay no longer.",
        }),

        ("exodus", 9, 29) => Some(Verse {
            content: "And Moses saide vnto him, Assoone as I am gone out of the citie, I will spread abroad my hands vnto the Lord, and the thunder shall cease, neither shall there be any more haile: that thou mayest know how that the earth is the Lords.",
        }),

        ("exodus", 9, 30) => Some(Verse {
            content: "But as for thee and thy seruants, I know that ye will not yet feare the Lord God.",
        }),

        ("exodus", 9, 31) => Some(Verse {
            content: "And the flaxe, and the barley was smitten: for the barley was in the eare, and the flaxe was bolled:",
        }),

        ("exodus", 9, 32) => Some(Verse {
            content: "But the wheat and the rye were not smitten: for they were not growen vp.",
        }),

        ("exodus", 9, 33) => Some(Verse {
            content: "And Moses went out of the city from Pharaoh, and spread abroad his hands vnto the Lord: and the thunders and haile ceased, and the raine was not powred vpon the earth.",
        }),

        ("exodus", 9, 34) => Some(Verse {
            content: "And when Pharaoh saw that the raine, and the haile and the thunders were ceased, hee sinned yet more, and hardened his heart, he and his seruants.",
        }),

        ("exodus", 9, 35) => Some(Verse {
            content: "And the heart of Pharaoh was hardened, neither would he let the children of Israel goe, as the Lord had spoken by Moses.",
        }),

        ("exodus", 10, 1) => Some(Verse {
            content: "And the Lord said vnto Moses, Goe in vnto Pharaoh: for I haue hardned his heart, and the heart of his seruants, that I might shew these my signes before him:",
        }),

        ("exodus", 10, 2) => Some(Verse {
            content: "And that thou mayest tell in the eares of thy sonne, and of thy sonnes sonne, what things I haue wrought in Egypt, and my signes which I haue done amongst them, that ye may know how that I am the Lord.",
        }),

        ("exodus", 10, 3) => Some(Verse {
            content: "And Moses and Aaron came in vnto Pharaoh, and saide vnto him, Thus saith the Lord God of the Hebrewes, How long wilt thou refuse to humble thy selfe before mee? Let my people goe, that they may serue me.",
        }),

        ("exodus", 10, 4) => Some(Verse {
            content: "Els, if thou refuse to let my people goe, behold, to morrow will I bring the locusts into thy coast.",
        }),

        ("exodus", 10, 5) => Some(Verse {
            content: "And they shall couer the face of the earth, that one cannot be able to see the earth, and they shall eate the residue of that which is escaped, which remaineth vnto you from the haile, and shall eate euery tree, which groweth for you out of the field.",
        }),

        ("exodus", 10, 6) => Some(Verse {
            content: "And they shall fill thy houses, and the houses of all thy seruants, and the houses of all the Egyptians, which neither thy fathers, nor thy fathers fathers haue seene, since the day that they were vpon the earth, vnto this day. And he turned himselfe, and went out from Pharaoh.",
        }),

        ("exodus", 10, 7) => Some(Verse {
            content: "And Pharaohs seruants said vnto him, How long shall this man be a snare vnto vs? Let the men goe, that they may serue the Lord their God: Knowest thou not yet, that Egypt is destroyed?",
        }),

        ("exodus", 10, 8) => Some(Verse {
            content: "And Moses and Aaron were brought againe vnto Pharaoh: and he said vnto them, Goe, serue the Lord your God: but who are they that shall goe?",
        }),

        ("exodus", 10, 9) => Some(Verse {
            content: "And Moses said, We wil goe with our yong, and with our old, with our sonnes and with our daughters, with our flockes and with our heards will we goe: for we must hold a feast vnto the Lord.",
        }),

        ("exodus", 10, 10) => Some(Verse {
            content: "And he said vnto them; Let the Lord bee so with you, as I will let you goe, and your litle ones. Looke to it, for euill is before you.",
        }),

        ("exodus", 10, 11) => Some(Verse {
            content: "Not so: goe now yee that are men, and serue the Lord, for that you did desire: and they were driuen out from Pharaohs presence.",
        }),

        ("exodus", 10, 12) => Some(Verse {
            content: "And the Lord said vnto Moses, Stretch out thine hand ouer the land of Egypt for the locusts, that they may come vp vpon the land of Egypt, and eate euery herbe of the land, euen all that the haile hath left.",
        }),

        ("exodus", 10, 13) => Some(Verse {
            content: "And Moses stretched forth his rod ouer the land of Egypt, and the Lord brought an East wind vpon the land all that day, and all that night: and when it was morning, the East wind brought the locusts.",
        }),

        ("exodus", 10, 14) => Some(Verse {
            content: "And the locusts went vp ouer all the land of Egypt, and rested in all the coasts of Egypt: very grieuous were they: before them there were no such locusts as they, neither after them shall be such.",
        }),

        ("exodus", 10, 15) => Some(Verse {
            content: "For they couered the face of the whole earth, so that the land was darkned, and they did eate euery herbe of the land, and all the fruit of the trees, which the haile had left, and there remained not any greene thing in the trees, or in the herbes of the field, through all the land of Egypt.",
        }),

        ("exodus", 10, 16) => Some(Verse {
            content: "Then Pharaoh called for Moses and Aaron in haste: and he said, I haue sinned against the Lord your God, and against you.",
        }),

        ("exodus", 10, 17) => Some(Verse {
            content: "Now therefore forgiue, I pray thee, my sinne onely this once, and intreat the Lord your God, that hee may take away from mee this death onely.",
        }),

        ("exodus", 10, 18) => Some(Verse {
            content: "And he went out from Pharaoh, and intreated the Lord.",
        }),

        ("exodus", 10, 19) => Some(Verse {
            content: "And the Lord turned a mighty strong West wind, which tooke away the locusts, and cast them into the red sea: there remained not one locust in all the coasts of Egypt.",
        }),

        ("exodus", 10, 20) => Some(Verse {
            content: "But the Lord hardened Pharaohs heart, so that hee would not let the children of Israel goe.",
        }),

        ("exodus", 10, 21) => Some(Verse {
            content: "And the Lord said vnto Moses, Stretch out thine hand toward heauen, that there may be darkenesse ouer the land of Egypt, euen darkenes which may be felt.",
        }),

        ("exodus", 10, 22) => Some(Verse {
            content: "And Moses stretched foorth his hand toward heauen: and there was a thicke darkenesse in all the land of Egypt three dayes.",
        }),

        ("exodus", 10, 23) => Some(Verse {
            content: "They saw not one another, neither rose any from his place for three dayes: but all the children of Israel had light in their dwellings.",
        }),

        ("exodus", 10, 24) => Some(Verse {
            content: "And Pharaoh called vnto Moses, and said, Goe ye, serue the Lord: onely let your flockes and your herds be stayed: let your litle ones also goe with you.",
        }),

        ("exodus", 10, 25) => Some(Verse {
            content: "And Moses saide, Thou must giue vs also sacrifices, and burnt offerings, that we may sacrifice vnto the Lord our God.",
        }),

        ("exodus", 10, 26) => Some(Verse {
            content: "Our cattell also shall goe with vs: there shall not an hoofe bee left behind: for thereof must we take to serue the Lord our God: and we knowe not with what wee must serue the Lord, vntill we come thither.",
        }),

        ("exodus", 10, 27) => Some(Verse {
            content: "But the Lord hardened Pharaohs heart, and he would not let them goe.",
        }),

        ("exodus", 10, 28) => Some(Verse {
            content: "And Pharaoh said vnto him, Get thee from me, take heed to thy selfe: see my face no more: for in that day thou seest my face, thou shalt die.",
        }),

        ("exodus", 10, 29) => Some(Verse {
            content: "And Moses said, Thou hast spoken well, I will see thy face againe no more.",
        }),

        ("exodus", 11, 1) => Some(Verse {
            content: "And the Lord said vnto Moses, Yet will I bring one plague more vpon Pharaoh, and vpon Egypt, afterwards hee will let you goe heuce: when hee shall let you goe, he shall surely thrust you out hence altogether.",
        }),

        ("exodus", 11, 2) => Some(Verse {
            content: "Speake now in the eares of the people, and let euery man borrowe of his neighbour, and euery woman of her neighbour, iewels of siluer, and iewels of gold.",
        }),

        ("exodus", 11, 3) => Some(Verse {
            content: "And the Lord gaue the people fauour in the sight of the Egyptians. Moreouer the man Moses was very great in the land of Egypt, in the sight of Pharaohs seruants, and in the sight of the people.",
        }),

        ("exodus", 11, 4) => Some(Verse {
            content: "And Moses said, Thus saith the Lord, about midnight will I goe out into the midst of Egypt.",
        }),

        ("exodus", 11, 5) => Some(Verse {
            content: "And all the first borne in the lande of Egypt shall die, from the first borne of Pharaoh, that sitteth vpon his throne, euen vnto the first borne of the maid seruant that is behind the mill, and all the first borne of beasts.",
        }),

        ("exodus", 11, 6) => Some(Verse {
            content: "And there shall bee a great crie throughout all the land of Egypt, such as there was none like it, nor shall bee like it any more.",
        }),

        ("exodus", 11, 7) => Some(Verse {
            content: "But against any of the children of Israel, shal not a dog moue his tongue, against man or beast: that ye may know how that the Lord doth put a difference betweene the Egyptians and Israel.",
        }),

        ("exodus", 11, 8) => Some(Verse {
            content: "And all these thy seruants shall come downe vnto me, and bow downe themselues vnto me, saying, Get thee out, and all the people that follow thee; and after that I wil goe out: and he went out from Pharaoh in a great anger.",
        }),

        ("exodus", 11, 9) => Some(Verse {
            content: "And the Lord said vnto Moses, Pharaoh shall not hearken vnto you, that my wonders may be multiplied in the land of Egypt.",
        }),

        ("exodus", 11, 10) => Some(Verse {
            content: "And Moses and Aaron did all these wonders before Pharaoh: and the Lord hardened Pharaohs heart, so that he would not let the children of Israel goe out of his land.",
        }),

        ("exodus", 12, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses and Aaron in the land of Egypt, saying,",
        }),

        ("exodus", 12, 2) => Some(Verse {
            content: "This moneth shalbe vnto you the beginning of moneths: it shall be the first moneth of the yeere to you.",
        }),

        ("exodus", 12, 3) => Some(Verse {
            content: "Speake ye vnto all the Congregation of Israel, saying, In the tenth day of this moneth they shall take to them euery man a lambe, according to the house of their fathers, a lambe for an house.",
        }),

        ("exodus", 12, 4) => Some(Verse {
            content: "And if the houshold be too little for the lambe, let him and his neighbour next vnto his house, take it according to the number of the soules: euery man according to his eating shall make your count for the lambe.",
        }),

        ("exodus", 12, 5) => Some(Verse {
            content: "Your lambe shall be without blemish, a male of the first yeere: yee shall take it out from the sheepe or from the goates.",
        }),

        ("exodus", 12, 6) => Some(Verse {
            content: "And ye shall keepe it vp vntill the fourteenth day of the same moneth: and the whole assembly of the congregation of Israel shall kill it in the euening.",
        }),

        ("exodus", 12, 7) => Some(Verse {
            content: "And they shall take of the blood and strike it on the two side postes, and on the vpper doore poste, of the houses wherin they shall eate it.",
        }),

        ("exodus", 12, 8) => Some(Verse {
            content: "And they shall eat the flesh in that night roste with fire, and vnleauened bread, and with bitter herbes they shall eate it.",
        }),

        ("exodus", 12, 9) => Some(Verse {
            content: "Eate not of it raw, nor sodden at all with water, but roste with fire: his head, with his legs, and with the purtenance thereof.",
        }),

        ("exodus", 12, 10) => Some(Verse {
            content: "And ye shall let nothing of it remaine vntill the morning: and that which remaineth of it vntill the morning, ye shall burne with fire.",
        }),

        ("exodus", 12, 11) => Some(Verse {
            content: "And thus shall ye eate it: with your loines girded, your shooes on your feet, and your staffe in your hand: and ye shall eate it in haste: it is the Lords Passeouer.",
        }),

        ("exodus", 12, 12) => Some(Verse {
            content: "For I will passe through the land of Egypt this night, and will smite all the first borne in the land of Egypt, both man & beast, and against all the gods of Egypt I will execute iudgement: I am the Lord.",
        }),

        ("exodus", 12, 13) => Some(Verse {
            content: "And the blood shall be to you for a token vpon the houses where you are: and when I see the blood, I will passe ouer you, and the plague shall not bee vpon you to destroy you, when I smite the land of Egypt.",
        }),

        ("exodus", 12, 14) => Some(Verse {
            content: "And this day shall be vnto you for a memoriall: and you shall keepe in a feast to the Lord, throughout your generations: you shall keepe it a feast by an ordinance for euer.",
        }),

        ("exodus", 12, 15) => Some(Verse {
            content: "Seuen dayes shall ye eate vnleauened bread, euen the first day yee shall put away leauen out of your houses: For whosoeuer eateth leauened bread, from the first day vntil the seuenth day, that soule shall be cut off from Israel.",
        }),

        ("exodus", 12, 16) => Some(Verse {
            content: "And in the first day there shalbe an holy conuocation, and in the seuenth day there shall be an holy conuocation to you: no maner of worke shalbe done in them, saue that which euery man must eate, that onely may bee done of you.",
        }),

        ("exodus", 12, 17) => Some(Verse {
            content: "And yee shall obserue the feast of vnleauened bread: for in this selfe same day haue I brought your armies out of the land of Egypt; therefore shall ye obserue this day in your generations, by an ordinance for euer.",
        }),

        ("exodus", 12, 18) => Some(Verse {
            content: "In the first moneth, on the fourteenth day of the moneth at euen, ye shall eate vnleauened bread vntill the one and twentieth day of the moneth at euen.",
        }),

        ("exodus", 12, 19) => Some(Verse {
            content: "Seuen dayes shall there bee no leauen found in your houses: for whosoeuer eateth that which is leauened, euen that soule shall be cut off from the congregation of Israel, whether he be a stranger, or borne in the land.",
        }),

        ("exodus", 12, 20) => Some(Verse {
            content: "Yee shall eate nothing leauened: in all your habitations shall ye eate vnleauened bread.",
        }),

        ("exodus", 12, 21) => Some(Verse {
            content: "Then Moses called for all the Elders of Israel, and said vnto them; Draw out and take you a lambe, according to your families, and kill the Passeouer.",
        }),

        ("exodus", 12, 22) => Some(Verse {
            content: "And ye shall take a bunch of hysope, and dip it in the blood that is in the bason, and strike the lintel and the two side postes with the blood that is in the bason: and none of you shall goe out at the doore of his house, vntill the morning.",
        }),

        ("exodus", 12, 23) => Some(Verse {
            content: "For the Lord wil passe through to smite the Egyptians: and when hee seeth the blood vpon the lintel, and on the two side-postes, the Lord will passe ouer the doore, and will not suffer the destroyer to come in vnto your houses to smite you.",
        }),

        ("exodus", 12, 24) => Some(Verse {
            content: "And ye shall obserue this thing for an ordinance to thee, and to thy sonnes for euer.",
        }),

        ("exodus", 12, 25) => Some(Verse {
            content: "And it shall come to passe when yee bee come to the land, which the Lord will giue you, according as he hath promised, that ye shall keepe this seruice.",
        }),

        ("exodus", 12, 26) => Some(Verse {
            content: "And it shall come to passe, when your children shall say vnto you, What meane you by this seruice?",
        }),

        ("exodus", 12, 27) => Some(Verse {
            content: "That ye shall say, It is the sacrifice of the Lords Passeouer, who passed ouer the houses of the children of Israel in Egypt, when he smote the Egyptians, and deliuered our houses. And the people bowed the head, and worshipped.",
        }),

        ("exodus", 12, 28) => Some(Verse {
            content: "And the children of Israel went away, and did as the Lord had commanded Moses and Aaron, so did they.",
        }),

        ("exodus", 12, 29) => Some(Verse {
            content: "And it came to passe that at midnight the Lord smote all the first borne in the land of Egypt, from the first borne of Pharaoh that sate on his throne, vnto the first borne of the captiue that was in the dungeon, and all the first borne of cattell.",
        }),

        ("exodus", 12, 30) => Some(Verse {
            content: "And Pharaoh rose vp in the night, hee and all his seruants, and all the Egyptians; and there was a great cry in Egypt: for there was not a house, where there was not one dead.",
        }),

        ("exodus", 12, 31) => Some(Verse {
            content: "And hee called for Moses and Aaron by night, and said, Rise vp, and get you forth from amongst my people, both you and the children of Israel: and goe, serue the Lord, as ye haue said.",
        }),

        ("exodus", 12, 32) => Some(Verse {
            content: "Also take your flockes and your heards, as ye haue said: and bee gone, and blesse me also.",
        }),

        ("exodus", 12, 33) => Some(Verse {
            content: "And the Egyptians were vrgent vpon the people that they might send them out of the land in haste: for they said, We be all dead men.",
        }),

        ("exodus", 12, 34) => Some(Verse {
            content: "And the people tooke their dough before it was leauened, their kneading troughes beeing bound vp in their clothes vpon their shoulders.",
        }),

        ("exodus", 12, 35) => Some(Verse {
            content: "And the children of Israel did according to the word of Moses: and they borrowed of the Egyptians iewels of siluer, and iewels of gold, and raiment.",
        }),

        ("exodus", 12, 36) => Some(Verse {
            content: "And the Lord gaue the people fauour in the sight of the Egyptians, so that they lent vnto them such things as they required: and they spoiled the Egyptians.",
        }),

        ("exodus", 12, 37) => Some(Verse {
            content: "And the children of Israel iourneyed from Rameses to Succoth, about sixe hundred thousand on foote that were men, beside children.",
        }),

        ("exodus", 12, 38) => Some(Verse {
            content: "And a mixed multitude went vp also with them, and flocks and heards, euen very much cattell.",
        }),

        ("exodus", 12, 39) => Some(Verse {
            content: "And they baked vnleauened cakes of the dough, which they brought forth out of Egypt; for it was not leauened: because they were thrust out of Egypt, and could not tarry, neither had they prepared for themselues any victuall.",
        }),

        ("exodus", 12, 40) => Some(Verse {
            content: "Now the soiourning of the children of Israel, who dwelt in Egypt, was foure hundred and thirtie yeeres.",
        }),

        ("exodus", 12, 41) => Some(Verse {
            content: "And it came to passe at the end of the foure hundred and thirtie yeeres, euen the selfe same day it came to passe, that all the hosts of the Lord went out from the land of Egypt.",
        }),

        ("exodus", 12, 42) => Some(Verse {
            content: "It is a night to be much obserued vnto the Lord, for bringing them out from the land of Egypt: This is that night of the Lord to be obserued of all the children of Israel, in their generations.",
        }),

        ("exodus", 12, 43) => Some(Verse {
            content: "And the Lord saide vnto Moses and Aaron, This is the ordinance of the Passeouer: there shall no stranger eate thereof.",
        }),

        ("exodus", 12, 44) => Some(Verse {
            content: "But euery mans seruant that is bought for money, when thou hast circumcised him, then shall he eate thereof.",
        }),

        ("exodus", 12, 45) => Some(Verse {
            content: "A forreiner, and an hired seruant shall not eate thereof.",
        }),

        ("exodus", 12, 46) => Some(Verse {
            content: "In one house shall it be eaten, thou shalt not carie foorth ought of the flesh abroad out of the house, neither shall ye breake a bone thereof.",
        }),

        ("exodus", 12, 47) => Some(Verse {
            content: "All the Congregation of Israel shall keepe it.",
        }),

        ("exodus", 12, 48) => Some(Verse {
            content: "And when a stranger shall soiourne with thee, and will keepe the Passeouer to the Lord, let all his males be circumcised, and then let him come neere, and keepe it: and he shall be as one that is borne in the land: for no vncircumcised person shall eate thereof.",
        }),

        ("exodus", 12, 49) => Some(Verse {
            content: "One law shall be to him that is home-borne, and vnto the stranger that soiourneth among you.",
        }),

        ("exodus", 12, 50) => Some(Verse {
            content: "Thus did all the children of Israel: as the Lord commanded Moses and Aaron, so did they.",
        }),

        ("exodus", 12, 51) => Some(Verse {
            content: "And it came to passe the selfe same day, that the Lord did bring the children of Israel out of the land of Egypt, by their armies.",
        }),

        ("exodus", 13, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 13, 2) => Some(Verse {
            content: "Sanctifie vnto me all the first borne, whatsoeuer openeth the wombe, among the children of Israel, both of man and of beast: it is mine.",
        }),

        ("exodus", 13, 3) => Some(Verse {
            content: "And Moses said vnto the people, Remember this day, in which yee came out from Egypt, out of the house of bondage: for by strength of hand the Lord brought you out from this place: there shall no leauened bread be eaten.",
        }),

        ("exodus", 13, 4) => Some(Verse {
            content: "This day came yee out, in the moneth Abib.",
        }),

        ("exodus", 13, 5) => Some(Verse {
            content: "And it shalbe when the Lord shall bring thee into the land of the Canaanites, and the Hittites, and the Amorites, and the Hiuites, and the Iebusites, which he sware vnto thy fathers to giue thee, a land flowing with milke and hony, that thou shalt keepe this seruice in this moneth.",
        }),

        ("exodus", 13, 6) => Some(Verse {
            content: "Seuen dayes thou shalt eate vnleauened bread, and in the seuenth day shall be a feast to the Lord.",
        }),

        ("exodus", 13, 7) => Some(Verse {
            content: "Unleauened bread shall be eaten seuen dayes: and there shall no leauened bread bee seene with thee: neither shall there be leauen seene with thee in all thy quarters.",
        }),

        ("exodus", 13, 8) => Some(Verse {
            content: "And thou shalt shew thy sonne in that day, saying, This is done because of that which the Lord did vnto mee, when I came forth out of Egypt.",
        }),

        ("exodus", 13, 9) => Some(Verse {
            content: "And it shall bee for a signe vnto thee, vpon thine hand, and for a memoriall betweene thine eyes, that the Lords law may be in thy mouth: for with a strong hande hath the Lord brought thee out of Egypt.",
        }),

        ("exodus", 13, 10) => Some(Verse {
            content: "Thou shalt therfore keepe this ordinance in his season from yeere to yere.",
        }),

        ("exodus", 13, 11) => Some(Verse {
            content: "And it shalbe when the Lord shall bring thee into the land of the Canaanites as he sware vnto thee, and to thy fathers, and shall giue it thee:",
        }),

        ("exodus", 13, 12) => Some(Verse {
            content: "That thou shalt set apart vnto the Lord all that openeth the matrix, and euery firstling that commeth of a beast, which thou hast, the males shall be the Lords.",
        }),

        ("exodus", 13, 13) => Some(Verse {
            content: "And euery firstling of an asse thou shalt redeeme with a lambe: and if thou wilt not redeeme it, then thou shalt breake his necke, and all the first borne of man amongst thy children shalt thou redeeme.",
        }),

        ("exodus", 13, 14) => Some(Verse {
            content: "And it shalbe when thy sonne asketh thee in time to come, saying, What is this? That thou shalt say vnto him; By strength of hand the Lord brought vs out from Egypt, from the house of bondage.",
        }),

        ("exodus", 13, 15) => Some(Verse {
            content: "And it came to passe when Pharaoh would hardly let vs goe, that the Lord slew all the first borne in the land of Egypt, both the first borne of man, and the first borne of beast: Therefore I sacrifice to the Lord all that openeth the matrix, being males: but all the first borne of my children I redeeme.",
        }),

        ("exodus", 13, 16) => Some(Verse {
            content: "And it shall be for a token vpon thine hand, and for frontlets betweene thine eyes. For by strength of hand the Lord brought vs foorth out of Egypt.",
        }),

        ("exodus", 13, 17) => Some(Verse {
            content: "And it came to passe when Pharaoh had let the people goe, that God led them not through the way of the land of the Philistines, although that was neere: For God saide, Lest peraduenture the people repent when they see warre, and they returne to Egypt:",
        }),

        ("exodus", 13, 18) => Some(Verse {
            content: "But God ledde the people about through the way of the wildernesse of the Red sea: and the children of Israel went vp harnessed out of the land of Egypt.",
        }),

        ("exodus", 13, 19) => Some(Verse {
            content: "And Moses tooke the bones of Ioseph with him: for hee had straitly sworne the children of Israel, saying; God will surely visite you, and ye shall cary vp my bones away hence with you.",
        }),

        ("exodus", 13, 20) => Some(Verse {
            content: "And they tooke their iourney from Succoth, and encamped in Etham, in the edge of the wildernesse.",
        }),

        ("exodus", 13, 21) => Some(Verse {
            content: "And the Lord went before them by day in a pillar of a cloud, to lead them the way, and by night in a pillar of fire, to giue them light to goe by day and night.",
        }),

        ("exodus", 13, 22) => Some(Verse {
            content: "He tooke not away the pillar of the cloud by day, nor the pillar of fire by night, from before the people.",
        }),

        ("exodus", 14, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 14, 2) => Some(Verse {
            content: "Speake vnto the children of Israel, that they turne and encampe before Pi-hahiroth, betweene Migdol and the sea, ouer against Baal-Zephon: before it shall ye encampe by the sea.",
        }),

        ("exodus", 14, 3) => Some(Verse {
            content: "For Pharaoh will say of the children of Israel, They are intangled in the land, the wildernesse hath shut them in.",
        }),

        ("exodus", 14, 4) => Some(Verse {
            content: "And I will harden Pharaohs heart, that he shall follow after them, and I will be honoured vpon Pharaoh, and vpon all his hoste, That the Egyptians may know that I am the Lord. And they did so.",
        }),

        ("exodus", 14, 5) => Some(Verse {
            content: "And it was told the King of Egypt, that the people fled: And the heart of Pharaoh and of his seruants was turned against the people, and they said, Why haue wee done this, that we haue let Israel goe from seruing vs?",
        }),

        ("exodus", 14, 6) => Some(Verse {
            content: "And hee made ready his charet, and tooke his people with him.",
        }),

        ("exodus", 14, 7) => Some(Verse {
            content: "And hee tooke sixe hundred chosen charets, and all the charets of Egypt, and captaines ouer euery one of them.",
        }),

        ("exodus", 14, 8) => Some(Verse {
            content: "And the Lord hardened the heart of Pharaoh King of Egypt, and he pursued after the children of Israel: and the children of Israel went out with an high hand.",
        }),

        ("exodus", 14, 9) => Some(Verse {
            content: "But the Egyptians pursued after them (all the horses and charets of Pharaoh, and his horsemen, and his army) and ouertooke them encamping by the sea, beside Pi-hahiroth before Baal-Zephon.",
        }),

        ("exodus", 14, 10) => Some(Verse {
            content: "And when Pharaoh drew nigh, the children of Israel lift vp their eyes, and behold, the Egyptians marched after them, and they were sore afraid: and the children of Israel lift vp their eyes, and beholde, the Egyptians marched after them, and they were sore afraid: and the children of Israel cried out vnto the Lord.",
        }),

        ("exodus", 14, 11) => Some(Verse {
            content: "And they said vnto Moses, Because there were no graues in Egypt, hast thou taken vs away to die in the wildernesse? Wherefore hast thou dealt thus with vs, to cary vs foorth out of Egypt?",
        }),

        ("exodus", 14, 12) => Some(Verse {
            content: "Is not this the word that wee did tell thee in Egypt, saying, Let vs alone, that we may serue the Egyptians? For it had bene better for vs to serue the Egyptians, then that wee should die in the wildernesse.",
        }),

        ("exodus", 14, 13) => Some(Verse {
            content: "And Moses saide vnto the people, Feare ye not, stand still, and see the saluation of the Lord, which he will shew to you to day: for the Egyptians whom ye haue seene to day, ye shall see them againe no more for euer.",
        }),

        ("exodus", 14, 14) => Some(Verse {
            content: "The Lord shall fight for you, and ye shall hold your peace.",
        }),

        ("exodus", 14, 15) => Some(Verse {
            content: "And the Lord saide vnto Moses, Wherefore criest thou vnto me? Speake vnto the children of Israel, that they goe forward.",
        }),

        ("exodus", 14, 16) => Some(Verse {
            content: "But lift thou vp thy rodde, and stretch out thine hand ouer the Sea, and diuide it: and the children of Israel shall goe on dry ground thorow the mids of the Sea.",
        }),

        ("exodus", 14, 17) => Some(Verse {
            content: "And I, beholde, I will harden the hearts of the Egyptians, and they shall follow them: and I will get mee honour vpon Pharaoh, and vpon all his hoste, vpon his charets, and vpon his horsemen.",
        }),

        ("exodus", 14, 18) => Some(Verse {
            content: "And the Egyptians shall know that I am the Lord, when I haue gotten me honour vpon Pharaoh, vpon his charets, and vpon his horsemen.",
        }),

        ("exodus", 14, 19) => Some(Verse {
            content: "And the Angel of God which went before the campe of Israel, remoued and went behind them, and the pillar of the cloud went from before their face, and stood behinde them.",
        }),

        ("exodus", 14, 20) => Some(Verse {
            content: "And it came betweene the campe of the Egyptians, and the campe of Israel, and it was a cloud and darkenesse to them, but it gaue light by night to these: so that the one came not neere the other all the night.",
        }),

        ("exodus", 14, 21) => Some(Verse {
            content: "And Moses stretched out his hand ouer the Sea, and the Lord caused the Sea to goe backe by a strong East winde all that night, and made the Sea dry land, and the waters were diuided.",
        }),

        ("exodus", 14, 22) => Some(Verse {
            content: "And the children of Israel went into the midst of the Sea vpon the dry ground, and the waters were a wall vnto them on their right hand, and on their left.",
        }),

        ("exodus", 14, 23) => Some(Verse {
            content: "And the Egyptians pursued, and went in after them, to the midst of the Sea, euen all Pharaohs horses, his charets and his horsemen.",
        }),

        ("exodus", 14, 24) => Some(Verse {
            content: "And it came to passe, that in the morning watch the Lord looked vnto the hoste of the Egyptians, through the pillar of fire, and of the cloude, and troubled the hoste of the Egyptians,",
        }),

        ("exodus", 14, 25) => Some(Verse {
            content: "And tooke off their charet wheeles, that they draue them heauily: So that the Egyptians said, Let vs flee from the face of Israel: for the Lord fighteth for them, against the Egyptians.",
        }),

        ("exodus", 14, 26) => Some(Verse {
            content: "And the Lord saide vnto Moses, Stretch out thine hand ouer the Sea, that the waters may come againe vpon the Egyptians, vpon their charets, and vpon their horsemen.",
        }),

        ("exodus", 14, 27) => Some(Verse {
            content: "And Moses stretched foorth his hand ouer the sea, and the sea returned to his strength when the morning appeared: and the Egyptians fled against it: and the Lord ouerthrew the Egyptians in the midst of the sea.",
        }),

        ("exodus", 14, 28) => Some(Verse {
            content: "And the waters returned, and couered the charets, and the horsemen, and all the hoste of Pharaoh that came into the sea after them: there remained not so much as one of them.",
        }),

        ("exodus", 14, 29) => Some(Verse {
            content: "But the children of Israel walked vpon drie land, in the midst of the sea, and the waters were a wall vnto them on their right hand, and on their left.",
        }),

        ("exodus", 14, 30) => Some(Verse {
            content: "Thus the Lord saued Israel that day out of the hand of the Egyptians: and Israel sawe the Egyptians dead vpon the sea shore.",
        }),

        ("exodus", 14, 31) => Some(Verse {
            content: "And Israel saw that great worke which the Lord did vpon the Egyptians: & the people feared the Lord, and beleeued the Lord, and his seruant Moses.",
        }),

        ("exodus", 15, 1) => Some(Verse {
            content: "Then sang Moses and the children of Israel this song vnto the Lord, and spake, saying, I will sing vnto the Lord: for he hath triumphed gloriously, the horse and his rider hath he throwen into the Sea.",
        }),

        ("exodus", 15, 2) => Some(Verse {
            content: "The Lord is my strength and song, and he is become my saluation: he is my God, and I will prepare him an habitation, my fathers God, and I wil exalt him.",
        }),

        ("exodus", 15, 3) => Some(Verse {
            content: "The Lord is a man of warre: the Lord is his Name.",
        }),

        ("exodus", 15, 4) => Some(Verse {
            content: "Pharaohs charets and his hoste hath he cast into the sea: his chosen captaines also are drowned in the red Sea.",
        }),

        ("exodus", 15, 5) => Some(Verse {
            content: "The depths haue couered them: they sanke into the bottome as a stone.",
        }),

        ("exodus", 15, 6) => Some(Verse {
            content: "Thy right hand, O Lord, is become glorious in power, thy right hand, O Lord, hath dashed in pieces the enemie.",
        }),

        ("exodus", 15, 7) => Some(Verse {
            content: "And in the greatnesse of thine excellencie thou hast ouerthrowen them, that rose vp against thee: thou sentest forth thy wrath, which consumed them as stubble.",
        }),

        ("exodus", 15, 8) => Some(Verse {
            content: "And with the blast of thy nostrils the waters were gathered together: the floods stood vpright as an heape, and the depths were congealed in the heart of the Sea.",
        }),

        ("exodus", 15, 9) => Some(Verse {
            content: "The enemie said, I will pursue, I wil ouertake, I wil diuide the spoile: my lust shall be satisfied vpon them: I will draw my sword, mine hand shall destroy them.",
        }),

        ("exodus", 15, 10) => Some(Verse {
            content: "Thou didst blow with thy wind, the sea couered them, they sanke as lead in the mighty waters.",
        }),

        ("exodus", 15, 11) => Some(Verse {
            content: "Who is like vnto thee, O Lord, amongst the gods? who is like thee, glorious in holinesse, fearefull in praises, doing wonders!",
        }),

        ("exodus", 15, 12) => Some(Verse {
            content: "Thou stretchedst out thy right hand, the earth swallowed them.",
        }),

        ("exodus", 15, 13) => Some(Verse {
            content: "Thou in thy mercie hast led forth the people which thou hast redeemed: thou hast guided them in thy strength vnto thy holy habitation.",
        }),

        ("exodus", 15, 14) => Some(Verse {
            content: "The people shall heare, and be afraid: sorrow shall take hold on the inhabitants of Palestina.",
        }),

        ("exodus", 15, 15) => Some(Verse {
            content: "Then the dukes of Edom shal be amased: the mighty men of Moab trembling shall take hold vpon them: all the inhabitants of Canaan shal melt away.",
        }),

        ("exodus", 15, 16) => Some(Verse {
            content: "Feare and dread shall fall vpon them, by the greatnesse of thine arme they shall be as still as a stone, till thy people passe ouer, O Lord, till the people passe ouer which thou hast purchased.",
        }),

        ("exodus", 15, 17) => Some(Verse {
            content: "Thou shalt bring them in, and plant them in the mountaine of thine inheritance, in the place, O Lord, which thou hast made for thee to dwell in, in the Sanctuary, O Lord, which thy hands haue established.",
        }),

        ("exodus", 15, 18) => Some(Verse {
            content: "The Lord shal reigne for euer and euer.",
        }),

        ("exodus", 15, 19) => Some(Verse {
            content: "For the horse of Pharaoh went in with his charets and with his horsemen into the sea, and the Lord brought againe the waters of the Sea vpon them: But the children of Israel went on drie land in the mids of the sea.",
        }),

        ("exodus", 15, 20) => Some(Verse {
            content: "And Miriam the prophetesse the sister of Aaron, tooke a timbrell in her hand, and all the women went out after her, with timbrels & with dances.",
        }),

        ("exodus", 15, 21) => Some(Verse {
            content: "And Miriam answered them, Sing ye to the Lord, for he hath triumphed gloriously: the horse and his rider hath he throwen into the sea.",
        }),

        ("exodus", 15, 22) => Some(Verse {
            content: "So Moses brought Israel from the red sea, and they went out into the wildernesse of Shur: and they went three dayes in the wildernesse, and found no water.",
        }),

        ("exodus", 15, 23) => Some(Verse {
            content: "And when they came to Marah, they could not drinke of the waters of Marah, for they were bitter: therefore the name of it was called Marah.",
        }),

        ("exodus", 15, 24) => Some(Verse {
            content: "And the people murmured against Moses, saying, What shall wee drinke?",
        }),

        ("exodus", 15, 25) => Some(Verse {
            content: "And he cried vnto the Lord: and the Lord shewed him a tree, which when hee had cast into the waters, the waters were made sweete: there he made a statute & an ordinance, and there he proued them,",
        }),

        ("exodus", 15, 26) => Some(Verse {
            content: "And said, If thou wilt diligently hearken to the voice of the Lord thy God, and wilt doe that which is right in his sight, and wilt giue eare to his Commandements, and keepe all his Statutes, I will put none of these diseases vpon thee, which I haue brought vpon the Egyptians: for I am the Lord that healeth thee.",
        }),

        ("exodus", 15, 27) => Some(Verse {
            content: "And they came to Elim: where were twelue wels of water, and threescore and ten palme trees, and they encamped there by the waters.",
        }),

        ("exodus", 16, 1) => Some(Verse {
            content: "And they tooke their iourney from Elim, and all the Congregation of the children of Israel came vnto the wildernesse of Sin, which is betweene Elim and Sinai, on the fifteenth day of the second moneth after their departing out of the land of Egypt.",
        }),

        ("exodus", 16, 2) => Some(Verse {
            content: "And the whole Congregation of the children of Israel murmured against Moses and Aaron in the wildernesse.",
        }),

        ("exodus", 16, 3) => Some(Verse {
            content: "And the children of Israel saide vnto them, Would to God wee had died by the hand of the Lord in the land of Egypt, when wee sate by the flesh pots, and when we did eate bread to the full: for ye haue brought vs forth into this wildernesse, to kill this whole assembly with hunger.",
        }),

        ("exodus", 16, 4) => Some(Verse {
            content: "Then said the Lord vnto Moses, Behold, I will raine bread from heauen for you: and the people shall goe out, and gather a certaine rate euery day, that I may proue them, whether they will walke in my Law, or no.",
        }),

        ("exodus", 16, 5) => Some(Verse {
            content: "And it shall come to passe, that on the sixt day, they shall prepare that which they bring in, and it shall be twice as much as they gather dayly.",
        }),

        ("exodus", 16, 6) => Some(Verse {
            content: "And Moses and Aaron said vnto all the children of Israel, At euen, then ye shall know that the Lord hath brought you out from the land of Egypt.",
        }),

        ("exodus", 16, 7) => Some(Verse {
            content: "And in the morning, then ye shall see the glory of the Lord, for that he heareth your murmurings against the Lord: And what are wee, that yee murmure against vs?",
        }),

        ("exodus", 16, 8) => Some(Verse {
            content: "And Moses said, This shalbe when the Lord shal giue you in the euening flesh to eate, and in the morning bread to the full: for that the Lord heareth your murmurings which ye murmure against him; and what are wee? your murmurings are not against vs, but against the Lord.",
        }),

        ("exodus", 16, 9) => Some(Verse {
            content: "And Moses spake vnto Aaron, Say vnto all the Congregation of the children of Israel, Come neere before the Lord: for hee hath heard your murmurings.",
        }),

        ("exodus", 16, 10) => Some(Verse {
            content: "And it came to passe as Aaron spake vnto the whole Congregation of the children of Israel, that they looked toward the wildernesse, and behold, the glory of the Lord appeared in the cloude.",
        }),

        ("exodus", 16, 11) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 16, 12) => Some(Verse {
            content: "I haue heard the murmurings of the children of Israel: Speake vnto them, saying, At euen ye shall eat flesh, and in the morning ye shalbe filled with bread: and ye shal know that I am the Lord your God.",
        }),

        ("exodus", 16, 13) => Some(Verse {
            content: "And it came to passe, that at euen the Quailes came vp, and couered the campe: and in the morning the dew lay round about the hoste.",
        }),

        ("exodus", 16, 14) => Some(Verse {
            content: "And when the dewe that lay was gone vp, behold, vpon the face of the wildernesse there lay a small round thing, as small as the hoare frost on the ground.",
        }),

        ("exodus", 16, 15) => Some(Verse {
            content: "And when the children of Israel saw it, they said one to another, It is Manna: for they wist not what it was. And Moses said vnto them, This is the bread which the Lord hath giuen you to eate.",
        }),

        ("exodus", 16, 16) => Some(Verse {
            content: "This is the thing which the Lord hath commanded: gather of it euery man according to his eating: an Omer for euery man, according to the number of your persons, take yee euery man for them which are in his tents.",
        }),

        ("exodus", 16, 17) => Some(Verse {
            content: "And the children of Israel did so, and gathered some more, some lesse.",
        }),

        ("exodus", 16, 18) => Some(Verse {
            content: "And when they did mete it with an Omer, he that gathered much, had nothing ouer, and he that gathered litle, had no lacke: they gathered euery man according to his eating.",
        }),

        ("exodus", 16, 19) => Some(Verse {
            content: "And Moses saide, Let no man leaue of it till the morning.",
        }),

        ("exodus", 16, 20) => Some(Verse {
            content: "Notwithstanding they hearkened not vnto Moses, but some of them left of it vntill the morning, and it bred wormes, and stanke: and Moses was wroth with them.",
        }),

        ("exodus", 16, 21) => Some(Verse {
            content: "And they gathered it euery morning, euery man according to his eating: and when the Sunne waxed hot, it melted.",
        }),

        ("exodus", 16, 22) => Some(Verse {
            content: "And it came to passe that on the sixt day they gathered twice as much bread, two Omers for one man: and all the rulers of the Congregation came and told Moses.",
        }),

        ("exodus", 16, 23) => Some(Verse {
            content: "And he said vnto them, This is that which the Lord hath said, To morrow is the rest of the holy Sabbath vnto the Lord: bake that which you will bake, to day, and seethe that ye will seethe, and that which remaineth ouer, lay vp for you to be kept vntill the morning.",
        }),

        ("exodus", 16, 24) => Some(Verse {
            content: "And they laid it vp till the morning, as Moses bade: and it did not stinke, neither was there any worme therein.",
        }),

        ("exodus", 16, 25) => Some(Verse {
            content: "And Moses saide, Eate that to day, for to day is a Sabbath vnto the Lord: to day yee shall not finde it in the field.",
        }),

        ("exodus", 16, 26) => Some(Verse {
            content: "Sixe dayes ye shall gather it, but on the seuenth day which is the Sabbath, in it there shall be none.",
        }),

        ("exodus", 16, 27) => Some(Verse {
            content: "And it came to passe, that there went out some of the people on the seuenth day for to gather, and they found none.",
        }),

        ("exodus", 16, 28) => Some(Verse {
            content: "And the Lord said vnto Moses, How long refuse yee to keepe my Commandements, and my Lawes?",
        }),

        ("exodus", 16, 29) => Some(Verse {
            content: "See, for that the Lord hath giuen you the Sabbath, therefore hee giueth you on the sixt day the bread of two dayes: abide yee euery man in his place: let no man goe out of his place on the seuenth day.",
        }),

        ("exodus", 16, 30) => Some(Verse {
            content: "So the people rested on the seuenth day.",
        }),

        ("exodus", 16, 31) => Some(Verse {
            content: "And the house of Israel called the name thereof Manna: and it was like Coriander seed, white: and the taste of it was like wafers made with hony.",
        }),

        ("exodus", 16, 32) => Some(Verse {
            content: "And Moses said, This is the thing which the Lord commandeth: Fill an Omer of it to bee kept for your generations, that they may see the bread wherewith I haue fed you in the wildernesse, when I brought you forth from the land of Egypt.",
        }),

        ("exodus", 16, 33) => Some(Verse {
            content: "And Moses sayd vnto Aaron, Take a pot, and put an Omer full of Manna therein, and lay it vp before the Lord, to be kept for your generations.",
        }),

        ("exodus", 16, 34) => Some(Verse {
            content: "As the Lord commaunded Moses, so Aaron layd it vp before the Testimonie, to be kept.",
        }),

        ("exodus", 16, 35) => Some(Verse {
            content: "And the children of Israel did eat Manna fortie yeeres, vntill they came to a land inhabited: they did eate Manna, vntill they came vnto the borders of the land of Canaan.",
        }),

        ("exodus", 16, 36) => Some(Verse {
            content: "Now an Omer is the tenth part of an Ephah.",
        }),

        ("exodus", 17, 1) => Some(Verse {
            content: "And all the Congregation of the children of Israel iourneyed from the wildernesse of Sin after their iourneys, according to the commandement of the Lord, and pitched in Rephidim: and there was no water for the people to drinke.",
        }),

        ("exodus", 17, 2) => Some(Verse {
            content: "Wherefore the people did chide with Moses and said, Giue vs water that wee may drinke. And Moses said vnto them, Why chide you with mee? Wherefore doe ye tempt the Lord ?", //intented space
        }),

        ("exodus", 17, 3) => Some(Verse {
            content: "And the people thirsted there for water, and the people murmured against Moses, and said, Wherefore is this that thou hast brought vs vp out of Egypt, to kill vs and our children, and our cattell with thirst?",
        }),

        ("exodus", 17, 4) => Some(Verse {
            content: "And Moses cried vnto the Lord, saying, What shall I doe vnto this people? They be almost ready to stone me.",
        }),

        ("exodus", 17, 5) => Some(Verse {
            content: "And the Lord said vnto Moses, Goe on before the people, and take with thee of the Elders of Israel: and thy rod wherewith thou smotest the riuer, take in thine hand, and goe.",
        }),

        ("exodus", 17, 6) => Some(Verse {
            content: "Behold, I will stand before thee there, vpon the rocke in Horeb, and thou shalt smite the rocke, and there shall come water out of it, that the people may drinke. And Moses did so, in the sight of the Elders of Israel.",
        }),

        ("exodus", 17, 7) => Some(Verse {
            content: "And hee called the name of the place Massah, and Meribah, because of the chiding of the children of Israel, and because they tempted the Lord, saying, Is the Lord amongst vs, or not?",
        }),

        ("exodus", 17, 8) => Some(Verse {
            content: "Then came Amalek, & fought with Israel in Rephidim.",
        }),

        ("exodus", 17, 9) => Some(Verse {
            content: "And Moses said vnto Ioshua, Choose vs out men, and goe out, fight with Amalek: to morrow I will stand on the top of the hill, with the rodde of God in mine hand.",
        }),

        ("exodus", 17, 10) => Some(Verse {
            content: "So Ioshua did as Moses had said to him, and fought with Amalek: and Moses, Aaron, and Hur went vp to the top of the hill.",
        }),

        ("exodus", 17, 11) => Some(Verse {
            content: "And it came to passe when Moses held vp his hand, that Israel preuailed: and when he let downe his hand, Amalek preuailed.",
        }),

        ("exodus", 17, 12) => Some(Verse {
            content: "But Moses hands were heauie, and they tooke a stone, and put it vnder him, and he sate thereon: and Aaron and Hur stayed vp his hands, the one on the one side, and the other on the other side, and his handes were steady vntill the going downe of the Sunne.",
        }),

        ("exodus", 17, 13) => Some(Verse {
            content: "And Ioshua discomfited Amalek, and his people, with the edge of the sword.",
        }),

        ("exodus", 17, 14) => Some(Verse {
            content: "And the Lord said vnto Moses, Write this for a memoriall in a booke, and rehearse it in the eares of Ioshua: for I will vtterly put out the remembrance of Amalek from vnder heauen.",
        }),

        ("exodus", 17, 15) => Some(Verse {
            content: "And Moses built an Altar, and called the name of it IEHOUAH Nissi.",
        }),

        ("exodus", 17, 16) => Some(Verse {
            content: "For he said, Because the Lord hath sworne that the Lord will haue warre with Amalek from generation to generation.",
        }),

        ("exodus", 18, 1) => Some(Verse {
            content: "When Iethro the Priest of Midian, Moses father in law, heard of all that God had done for Moses, and for Israel his people, and that the Lord had brought Israel out of Egypt:",
        }),

        ("exodus", 18, 2) => Some(Verse {
            content: "Then Iethro Moses father in law tooke Zipporah Moses wife, after he had sent her backe,",
        }),

        ("exodus", 18, 3) => Some(Verse {
            content: "And her two sonnes, of which the name of the one was Gershom: for he said, I haue bene an alien in a strange land.",
        }),

        ("exodus", 18, 4) => Some(Verse {
            content: "And the name of the other was Eliezer: for the God of my father, said he, was mine helpe, and deliuered me from the sword of Pharaoh.",
        }),

        ("exodus", 18, 5) => Some(Verse {
            content: "And Iethro Moses father in law came with his sonnes and his wife vnto Moses into the wildernes, where he encamped at the mount of God.",
        }),

        ("exodus", 18, 6) => Some(Verse {
            content: "And he said vnto Moses, I thy father in law Iethro am come vnto thee, and thy wife, and her two sonnes with her.",
        }),

        ("exodus", 18, 7) => Some(Verse {
            content: "And Moses went out to meete his father in law, and did obeysance, and kissed him: and they asked each other of their welfare, and they came into the tent.",
        }),

        ("exodus", 18, 8) => Some(Verse {
            content: "And Moses told his father in law, all that the Lord had done vnto Pharaoh, and to the Egyptians for Israels sake, and all the trauaile that had come vpon them by the way, and how the Lord deliuered them.",
        }),

        ("exodus", 18, 9) => Some(Verse {
            content: "And Iethro reioyced for all the goodnesse which the Lord had done to Israel: whom he had deliuered out of the hand of the Egyptians.",
        }),

        ("exodus", 18, 10) => Some(Verse {
            content: "And Iethro said, Blessed be the Lord, who hath deliuered you out of the hand of the Egyptians, and out of the hand of Pharaoh, who hath deliuered the people from vnder the hand of the Egyptians.",
        }),

        ("exodus", 18, 11) => Some(Verse {
            content: "Now I know that the Lord is greater then all gods: for in the thing wherein they dealt proudly, hee was aboue them.",
        }),

        ("exodus", 18, 12) => Some(Verse {
            content: "And Iethro, Moses father in law, tooke a burnt offering and sacrifices for God: and Aaron came, and all the Elders of Israel, to eat bread with Moses father in law before God.",
        }),

        ("exodus", 18, 13) => Some(Verse {
            content: "And it came to passe on the morrow, that Moses sate to iudge the people: and the people stood by Moses, from the morning vnto the euening.",
        }),

        ("exodus", 18, 14) => Some(Verse {
            content: "And when Moses father in law saw all that he did to the people, he said, What is this thing that thou doest to the people? Why sittest thou thy selfe alone, and all the people stand by thee from morning vnto euen?",
        }),

        ("exodus", 18, 15) => Some(Verse {
            content: "And Moses said vnto his father in law, Because the people come vnto me to enquire of God.",
        }),

        ("exodus", 18, 16) => Some(Verse {
            content: "When they haue a matter, they come vnto mee, and I iudge betweene one and another, and I doe make them know the statutes of God and his Lawes.",
        }),

        ("exodus", 18, 17) => Some(Verse {
            content: "And Moses father in law saide vnto him, The thing that thou doest, is not good.",
        }),

        ("exodus", 18, 18) => Some(Verse {
            content: "Thou wilt surely weare away, both thou, and this people that is with thee: for this thing is too heauy for thee; thou art not able to performe it thy selfe alone.",
        }),

        ("exodus", 18, 19) => Some(Verse {
            content: "Hearken now vnto my voyce, I will giue thee counsell, and God shall be with thee: Be thou for the people to Godward, that thou mayest bring the causes vnto God:",
        }),

        ("exodus", 18, 20) => Some(Verse {
            content: "And thou shalt teach them ordinances and lawes, and shalt shew them the way wherein they must walke, and the worke that they must doe.",
        }),

        ("exodus", 18, 21) => Some(Verse {
            content: "Moreouer thou shalt prouide out of all the people able men, such as feare God, men of trueth, hating couetousnesse, and place such ouer them, to bee rulers of thousands, and rulers of hundreds, rulers of fifties, and rulers of tennes.",
        }),

        ("exodus", 18, 22) => Some(Verse {
            content: "And let them iudge the people at all seasons: and it shall bee that euery great matter they shall bring vnto thee, but euery small matter they shal iudge: so shall it be easier for thy selfe, and they shall beare the burden with thee.",
        }),

        ("exodus", 18, 23) => Some(Verse {
            content: "If thou shalt doe this thing, and God command thee so, then thou shalt bee able to endure, and all this people shall also goe to their place in peace.",
        }),

        ("exodus", 18, 24) => Some(Verse {
            content: "So Moses hearkened to the voice of his father in law, and did all that he had said.",
        }),

        ("exodus", 18, 25) => Some(Verse {
            content: "And Moses chose able men out of all Israel, and made them heads ouer the people, rulers of thousands, rulers of hundreds, rulers of fifties, and rulers of tennes.",
        }),

        ("exodus", 18, 26) => Some(Verse {
            content: "And they iudged the people at all seasons: the hard causes they brought vnto Moses, but euery small matter they iudged themselues.",
        }),

        ("exodus", 18, 27) => Some(Verse {
            content: "And Moses let his father in law depart, and he went his way into his owne land.",
        }),

        ("exodus", 19, 1) => Some(Verse {
            content: "In the third moneth when the children of Israel were gone forth out of the land of Egypt, the same day came they into the wildernesse of Sinai.",
        }),

        ("exodus", 19, 2) => Some(Verse {
            content: "For they were departed from Rephidim, and were come to the desert of Sinai, and had pitched in the wildernesse, and there Israel camped before the mount.",
        }),

        ("exodus", 19, 3) => Some(Verse {
            content: "And Moses went vp vnto God: and the Lord called vnto him out of the mountaine, saying, Thus shalt thou say to the house of Iacob, and tell the children of Israel:",
        }),

        ("exodus", 19, 4) => Some(Verse {
            content: "Ye haue seene what I did vnto the Egyptians, and how I bare you on Eagles wings, and brought you vnto my selfe.",
        }),

        ("exodus", 19, 5) => Some(Verse {
            content: "Now therfore if ye will obey my voice indeed, and keepe my couenant, then ye shall be a peculiar treasure vnto me aboue all people: for all the earth is mine.",
        }),

        ("exodus", 19, 6) => Some(Verse {
            content: "And ye shall be vnto me a kingdome of Priestes, and an holy nation. These are the wordes which thou shalt speake vnto the children of Israel.",
        }),

        ("exodus", 19, 7) => Some(Verse {
            content: "And Moses came and called for the Elders of the people, and layd before their faces all these wordes which the Lord commanded him.",
        }),

        ("exodus", 19, 8) => Some(Verse {
            content: "And all the people answered together, and said, All that the Lord hath spoken, we will doe. And Moses returned the wordes of the people vnto the Lord.",
        }),

        ("exodus", 19, 9) => Some(Verse {
            content: "And the Lord said vnto Moses, Loe, I come vnto thee in a thicke cloud, that the people may heare when I speake with thee, and beleeue thee for euer: And Moses told the wordes of the people vnto the Lord.",
        }),

        ("exodus", 19, 10) => Some(Verse {
            content: "And the Lord saide vnto Moses, Goe vnto the people, and sanctifie them to day and to morrow, and let them wash their clothes.",
        }),

        ("exodus", 19, 11) => Some(Verse {
            content: "And be ready against the thirde day: for the third day the Lord will come downe in the sight of all the people, vpon mount Sinai.",
        }),

        ("exodus", 19, 12) => Some(Verse {
            content: "And thou shalt set bounds vnto the people round about, saying, Take heed to your selues, that ye goe not vp into the mount, or touch the border of it: whosoeuer toucheth the mount, shall be surely put to death.",
        }),

        ("exodus", 19, 13) => Some(Verse {
            content: "There shall not a hand touch it, but he shall surely be stoned or shot thorow, whether it be beast, or man, it shall not liue: when the trumpet soundeth long, they shall come vp to the mount.",
        }),

        ("exodus", 19, 14) => Some(Verse {
            content: "And Moses went downe from the mount vnto the people, and sanctified the people; and they washed their clothes.",
        }),

        ("exodus", 19, 15) => Some(Verse {
            content: "And hee said vnto the people, Be ready against the third day: come not at your wiues.",
        }),

        ("exodus", 19, 16) => Some(Verse {
            content: "And it came to passe on the third day in the morning, that there were thunders and lightnings, and a thicke cloud vpon the mount, and the voyce of the trumpet exceeding lowd, so that all the people that was in the campe, trembled.",
        }),

        ("exodus", 19, 17) => Some(Verse {
            content: "And Moses brought foorth the people out of the campe to meete with God, and they stood at the nether part of the mount.",
        }),

        ("exodus", 19, 18) => Some(Verse {
            content: "And mount Sinai was altogether on a smoke, because the Lord descended vpon it in fire: and the smoke thereof ascended as the smoke of a furnace, and the whole mount quaked greatly.",
        }),

        ("exodus", 19, 19) => Some(Verse {
            content: "And when the voyce of the trumpet sounded long, and waxed lowder and lowder, Moses spake, and God answered him by a voyce.",
        }),

        ("exodus", 19, 20) => Some(Verse {
            content: "And the Lord came downe vpon mount Sinai, on the top of the mount: and the Lord called Moses vp to the top of the mount, and Moses went vp.",
        }),

        ("exodus", 19, 21) => Some(Verse {
            content: "And the Lord said vnto Moses, Goe downe, charge the people, lest they breake thorow vnto the Lord to gaze, and many of them perish.",
        }),

        ("exodus", 19, 22) => Some(Verse {
            content: "And let the Priestes also which come neere to the Lord, sanctifie themselues, lest the Lord breake foorth vpon them.",
        }),

        ("exodus", 19, 23) => Some(Verse {
            content: "And Moses said vnto the Lord, The people cannot come vp to mount Sinai: for thou chargedst vs, saying, Set bounds about the mount, and sanctifie it.",
        }),

        ("exodus", 19, 24) => Some(Verse {
            content: "And the Lord said vnto him, Away, get thee downe, and thou shalt come vp, thou, and Aaron with thee: but let not the Priestes and the people breake through, to come vp vnto the Lord, lest hee breake foorth vpon them.",
        }),

        ("exodus", 19, 25) => Some(Verse {
            content: "So Moses went downe vnto the people, and spake vnto them.",
        }),

        ("exodus", 20, 1) => Some(Verse {
            content: "And God spake all these words, saying,",
        }),

        ("exodus", 20, 2) => Some(Verse {
            content: "I am the Lord thy God, which haue brought thee out of the land of Egypt, out of the house of bondage:",
        }),

        ("exodus", 20, 3) => Some(Verse {
            content: "Thou shalt haue no other Gods before me.",
        }),

        ("exodus", 20, 4) => Some(Verse {
            content: "Thou shalt not make vnto thee any grauen Image, or any likenesse of any thing that is in heauen aboue, or that is in the earth beneath, or that is in the water vnder the earth.",
        }),

        ("exodus", 20, 5) => Some(Verse {
            content: "Thou shalt not bow downe thy selfe to them, nor serue them: For I the Lord thy God am a iealous God, visiting the iniquitie of the fathers vpon the children, vnto the thirde and fourth generation of them that hate me:",
        }),

        ("exodus", 20, 6) => Some(Verse {
            content: "And shewing mercy vnto thousands of them that loue mee, and keepe my Commandements.",
        }),

        ("exodus", 20, 7) => Some(Verse {
            content: "Thou shalt not take the Name of the Lord thy God in vaine: for the Lord will not holde him guiltlesse, that taketh his Name in vaine.",
        }),

        ("exodus", 20, 8) => Some(Verse {
            content: "Remember the Sabbath day, to keepe it holy.",
        }),

        ("exodus", 20, 9) => Some(Verse {
            content: "Sixe dayes shalt thou labour, and doe all thy worke:",
        }),

        ("exodus", 20, 10) => Some(Verse {
            content: "But the seuenth day is the Sabbath of the Lord thy God: in it thou shalt not doe any worke, thou, nor thy sonne, nor thy daughter, thy man seruant, nor thy mayd seruant, nor thy cattell, nor thy stranger that is within thy gates:",
        }),

        ("exodus", 20, 11) => Some(Verse {
            content: "For in sixe dayes the Lord made heauen and earth, the sea, and all that in them is, and rested the seuenth day: wherefore the Lord blessed the Sabbath day, and halowed it.",
        }),

        ("exodus", 20, 12) => Some(Verse {
            content: "Honour thy father and thy mother: that thy dayes may bee long vpon the land, which the Lord thy God giueth thee.",
        }),

        ("exodus", 20, 13) => Some(Verse {
            content: "Thou shalt not kill.",
        }),

        ("exodus", 20, 14) => Some(Verse {
            content: "Thou shalt not commit adultery.",
        }),

        ("exodus", 20, 15) => Some(Verse {
            content: "Thou shalt not steale.",
        }),

        ("exodus", 20, 16) => Some(Verse {
            content: "Thou shalt not beare false witnes against thy neighbour.",
        }),

        ("exodus", 20, 17) => Some(Verse {
            content: "Thou shalt not couet thy neighbours house, thou shalt not couet thy neighbours wife, nor his man seruant, nor his maid seruant, nor his oxe, nor his asse, nor any thing that is thy neighbours.",
        }),

        ("exodus", 20, 18) => Some(Verse {
            content: "And all the people saw the thundrings, and the lightnings, and the noise of the trumpet, and the mountaine smoking: and when the people saw it, they remooued, and stood a farre off.",
        }),

        ("exodus", 20, 19) => Some(Verse {
            content: "And they saide vnto Moses, Speake thou with vs, and wee will heare: But let not God speake with vs, lest we die.",
        }),

        ("exodus", 20, 20) => Some(Verse {
            content: "And Moses said vnto the people, Feare not: for God is come to prooue you, and that his feare may bee before your faces, that ye sinne not.",
        }),

        ("exodus", 20, 21) => Some(Verse {
            content: "And the people stood afarre off, and Moses drew neere vnto the thicke darkenes, where God was.",
        }),

        ("exodus", 20, 22) => Some(Verse {
            content: "And the Lord said vnto Moses, Thus thou shalt say vnto the children of Israel, Yee haue seene that I haue talked with you from heauen.",
        }),

        ("exodus", 20, 23) => Some(Verse {
            content: "Ye shall not make with me gods of siluer, neither shall ye make vnto you gods of gold.",
        }),

        ("exodus", 20, 24) => Some(Verse {
            content: "An Altar of earth thou shalt make vnto me, and shalt sacrifice thereon thy burnt offerings, and thy peace offerings, thy sheepe, and thine oxen: In all places where I record my Name, I will come vnto thee, and I will blesse thee.",
        }),

        ("exodus", 20, 25) => Some(Verse {
            content: "And if thou wilt make mee an Altar of stone, thou shalt not build it of hewen stone: for if thou lift vp thy toole vpon it, thou hast polluted it.",
        }),

        ("exodus", 20, 26) => Some(Verse {
            content: "Neither shalt thou goe vp by steps vnto mine Altar, that thy nakednesse be not discouered thereon.",
        }),

        ("exodus", 21, 1) => Some(Verse {
            content: "Now these are the Iudgements which thou shalt set before them.",
        }),

        ("exodus", 21, 2) => Some(Verse {
            content: "If thou buy an Hebrew seruant, sixe yeeres he shall serue, and in the seuenth he shall goe out free for nothing.",
        }),

        ("exodus", 21, 3) => Some(Verse {
            content: "If he came in by himselfe, he shal goe out by himselfe: if he were married, then his wife shall goe out with him.",
        }),

        ("exodus", 21, 4) => Some(Verse {
            content: "If his master haue giuen him a wife, and she haue borne him sonnes or daughters; the wife and her children shall be her masters, and he shall go out by himselfe.",
        }),

        ("exodus", 21, 5) => Some(Verse {
            content: "And if the seruant shall plainely say, I loue my master, my wife, and my children, I will not goe out free:",
        }),

        ("exodus", 21, 6) => Some(Verse {
            content: "Then his master shall bring him vnto the Iudges, hee shall also bring him to the doore, or vnto the doore post, and his master shall boare his eare through with an aule, and he shall serue him for euer.",
        }),

        ("exodus", 21, 7) => Some(Verse {
            content: "And if a man sell his daughter to be a mayd seruant, shee shall not goe out as the men seruants doe.",
        }),

        ("exodus", 21, 8) => Some(Verse {
            content: "If she please not her master, who hath betrothed her to himselfe, then shall he let her be redeemed: To sell her vnto a strange nation hee shall haue no power, seeing he hath dealt deceitfully with her.",
        }),

        ("exodus", 21, 9) => Some(Verse {
            content: "And if he haue betrothed her vnto his sonne, he shall deale with her after the maner of daughters.",
        }),

        ("exodus", 21, 10) => Some(Verse {
            content: "If he take him another wife, her food, her rayment, and her duety of mariage shall he not diminish.",
        }),

        ("exodus", 21, 11) => Some(Verse {
            content: "And if he doe not these three vnto her, then shall she goe out free without money.",
        }),

        ("exodus", 21, 12) => Some(Verse {
            content: "He that smiteth a man, so that he die, shalbe surely put to death.",
        }),

        ("exodus", 21, 13) => Some(Verse {
            content: "And if a man lye not in wait, but God deliuer him into his hand, then I will appoint thee a place whither hee shall flee:",
        }),

        ("exodus", 21, 14) => Some(Verse {
            content: "But if a man come presumptuously vpon his neighbour to slay him with guile, thou shalt take him from mine Altar, that he may die.",
        }),

        ("exodus", 21, 15) => Some(Verse {
            content: "And he that smiteth his father, or his mother, shall bee surely put to death.",
        }),

        ("exodus", 21, 16) => Some(Verse {
            content: "And he that stealeth a man, and selleth him, or if he be found in his hand, he shall surely be put to death.",
        }),

        ("exodus", 21, 17) => Some(Verse {
            content: "And hee that curseth his father or his mother, shall surely bee put to death.",
        }),

        ("exodus", 21, 18) => Some(Verse {
            content: "And if men striue together, and one smite another with a stone, or with his fist, and he die not, but keepeth his bed:",
        }),

        ("exodus", 21, 19) => Some(Verse {
            content: "If hee rise againe, and walke abroad vpon his staffe, then shall hee that smote him, be quit: onely he shall pay for the losse of his time, and shall cause him to be throughly healed.",
        }),

        ("exodus", 21, 20) => Some(Verse {
            content: "And if a man smite his seruant, or his mayd, with a rod, and hee die vnder his hand, hee shall bee surely punished:",
        }),

        ("exodus", 21, 21) => Some(Verse {
            content: "Notwithstanding, if he continue a day or two, hee shall not be punished, for he is his money.",
        }),

        ("exodus", 21, 22) => Some(Verse {
            content: "If men striue, and hurt a woman with child, so that her fruit depart from her, and yet no mischiefe follow, he shalbe surely punished, according as the womans husband will lay vpon him, and hee shall pay as the Iudges determine.",
        }),

        ("exodus", 21, 23) => Some(Verse {
            content: "And if any mischiefe follow, then thou shalt giue life for life,",
        }),

        ("exodus", 21, 24) => Some(Verse {
            content: "Eye for eye, tooth for tooth, hand for hand, foote for foote,",
        }),

        ("exodus", 21, 25) => Some(Verse {
            content: "Burning for burning, wound for wound, stripe for stripe.",
        }),

        ("exodus", 21, 26) => Some(Verse {
            content: "And if a man smite the eye of his seruant, or the eye of his mayd, that it perish, hee shall let him goe free for his eyes sake.",
        }),

        ("exodus", 21, 27) => Some(Verse {
            content: "And if he smite out his man seruants tooth, or his mayde seruants tooth, hee shal let him goe free for his tooths sake.",
        }),

        ("exodus", 21, 28) => Some(Verse {
            content: "If an oxe gore a man, or a woman, that they die, then the oxe shal be surely stoned, and his flesh shall not be eaten: but the owner of the oxe shall be quitte.",
        }),

        ("exodus", 21, 29) => Some(Verse {
            content: "But if the oxe were wont to push with his horne in time past, and it hath bene testified to his owner, and he hath not kept him in, but that he hath killed a man or a woman; the oxe shall be stoned, and his owner also shall bee put to death.",
        }),

        ("exodus", 21, 30) => Some(Verse {
            content: "If there be layed on him a summe of money, then he shall giue for the ransome of his life, whatsoeuer is layd vpon him.",
        }),

        ("exodus", 21, 31) => Some(Verse {
            content: "Whether hee haue gored a sonne, or haue gored a daughter, according to this iudgement shall it bee done vnto him.",
        }),

        ("exodus", 21, 32) => Some(Verse {
            content: "If the oxe shall push a man seruant, or a mayd seruant, hee shall giue vnto their master thirty shekels, and the oxe shalbe stoned.",
        }),

        ("exodus", 21, 33) => Some(Verse {
            content: "And if a man shall open a pit, or if a man shall digge a pit, and not couer it, and an oxe or an asse fall therein:",
        }),

        ("exodus", 21, 34) => Some(Verse {
            content: "The owner of the pit shall make it good, and giue money vnto the owner of them, and the dead beast shalbe his.",
        }),

        ("exodus", 21, 35) => Some(Verse {
            content: "And if one mans oxe hurt anothers, that he die, then they shall sell the liue oxe, and diuide the money of it, and the dead oxe also they shall diuide.",
        }),

        ("exodus", 21, 36) => Some(Verse {
            content: "Or if it bee knowen that the oxe hath vsed to push in time past, and his owner hath not kept him in, hee shall surely pay oxe for oxe, and the dead shall be his owne.",
        }),

        ("exodus", 22, 1) => Some(Verse {
            content: "If a man shal steale an oxe, or a sheepe, and kill it, or sell it; he shall restore fiue oxen for an oxe, and foure sheepe for a sheepe.",
        }),

        ("exodus", 22, 2) => Some(Verse {
            content: "If a thiefe bee found breaking vp, and be smitten that he die, there shal no blood be shed for him.",
        }),

        ("exodus", 22, 3) => Some(Verse {
            content: "If the Sunne be risen vpon him, there shall be blood shed for him: for hee should make full restitution: if he haue nothing, then he shall bee sold for his theft.",
        }),

        ("exodus", 22, 4) => Some(Verse {
            content: "If the theft be certainely found in his hand aliue, whether it bee oxe or asse, or sheepe, he shall restore double.",
        }),

        ("exodus", 22, 5) => Some(Verse {
            content: "If a man shall cause a field or vineyard to be eaten, and shall put in his beast, and shall feede in another mans field: of the best of his owne field, and of the best of his owne vineyard shall he make restitution.",
        }),

        ("exodus", 22, 6) => Some(Verse {
            content: "If fire breake out, and catch in thornes, so that the stackes of corne, or the standing corne, or the field be consumed therewith; hee that kindled the fire, shall surely make restitution.",
        }),

        ("exodus", 22, 7) => Some(Verse {
            content: "If a man shal deliuer vnto his neighbour money or stuffe to keepe, and it be stollen out of the mans house; if the thiefe be found, let him pay double.",
        }),

        ("exodus", 22, 8) => Some(Verse {
            content: "If the thiefe be not found, then the master of the house shall be brought vnto the Iudges, to see whether he haue put his hande vnto his neighbours goods.",
        }),

        ("exodus", 22, 9) => Some(Verse {
            content: "For all maner of trespasse, whether it be for oxe, for asse, for sheepe, for raiment, or for any maner of lost thing, which another challengeth to be his: the cause of both parties shall come before the Iudges, and whome the Iudges shall condemne, he shall pay double vnto his neighbour.",
        }),

        ("exodus", 22, 10) => Some(Verse {
            content: "If a man deliuer vnto his neighbour an asse, or an oxe, or a sheepe, or any beast to keepe, and it die, or be hurt, or driuen away, no man seeing it,",
        }),

        ("exodus", 22, 11) => Some(Verse {
            content: "Then shall an othe of the Lord be betweene them both, that hee hath not put his hand vnto his neighbours goods: and the owner of it shall accept thereof, and he shall not make it good.",
        }),

        ("exodus", 22, 12) => Some(Verse {
            content: "And if it be stollen from him, he shall make restitution vnto the owner thereof.",
        }),

        ("exodus", 22, 13) => Some(Verse {
            content: "If it be torne in pieces, then let him bring it for witnesse, and hee shall not make good that which was torne.",
        }),

        ("exodus", 22, 14) => Some(Verse {
            content: "And if a man borrowe ought of his neighbour, and it be hurt, or die, the owner thereof being not with it, he shall surely make it good.",
        }),

        ("exodus", 22, 15) => Some(Verse {
            content: "But if the owner thereof be with it, he shall not make it good: If it bee an hired thing, it came for his hire.",
        }),

        ("exodus", 22, 16) => Some(Verse {
            content: "And if a man entice a maide that is not betrothed, and lie with her, he shall surely endow her to be his wife.",
        }),

        ("exodus", 22, 17) => Some(Verse {
            content: "If her father vtterly refuse to giue her vnto him, he shall pay money according to the dowrie of virgins.",
        }),

        ("exodus", 22, 18) => Some(Verse {
            content: "Thou shalt not suffer a witch to liue.",
        }),

        ("exodus", 22, 19) => Some(Verse {
            content: "Whosoeuer lieth with a beast, shall surely be put to death.",
        }),

        ("exodus", 22, 20) => Some(Verse {
            content: "Hee that sacrificeth vnto any god saue vnto the Lord onely, hee shall be vtterly destroyed.",
        }),

        ("exodus", 22, 21) => Some(Verse {
            content: "Thou shalt neither vexe a stranger, nor oppresse him: for ye were strangers in the land of Egypt.",
        }),

        ("exodus", 22, 22) => Some(Verse {
            content: "Yee shall not afflict any widow, or fatherlesse child.",
        }),

        ("exodus", 22, 23) => Some(Verse {
            content: "If thou afflict them in any wise, and they crie at all vnto mee, I will surely heare their crie.",
        }),

        ("exodus", 22, 24) => Some(Verse {
            content: "And my wrath shall waxe hote, and I will kill you with the sword: and your wiues shall be widowes, and your children fatherlesse.",
        }),

        ("exodus", 22, 25) => Some(Verse {
            content: "If thou lend money to any of my people that is poore by thee, thou shalt not be to him as an vsurer, neither shalt thou lay vpon him vsurie.",
        }),

        ("exodus", 22, 26) => Some(Verse {
            content: "If thou at all take thy neighbors raiment to pledge, thou shalt deliuer it vnto him by that the sun goeth downe.",
        }),

        ("exodus", 22, 27) => Some(Verse {
            content: "For that is his couering onely, it is his raiment for his skinne: wherein shal he sleepe? and it shal come to passe, when he crieth vnto mee, that I will heare: for I am gracious.",
        }),

        ("exodus", 22, 28) => Some(Verse {
            content: "Thou shalt not reuile the Gods, nor curse the ruler of thy people.",
        }),

        ("exodus", 22, 29) => Some(Verse {
            content: "Thou shalt not delay to offer the first of thy ripe fruits, and of thy liquors: the first borne of thy sonnes shalt thou giue vnto me.",
        }),

        ("exodus", 22, 30) => Some(Verse {
            content: "Likewise shalt thou do with thine oxen, and with thy sheepe: seuen dayes it shall be with his damme, on the eight day thou shalt giue it me.",
        }),

        ("exodus", 22, 31) => Some(Verse {
            content: "And ye shall be holy men vnto me: neither shall ye eate any flesh that is torne of beasts in the field: yee shall cast it to the dogs.",
        }),

        ("exodus", 23, 1) => Some(Verse {
            content: "Thou shalt not raise a false report: put not thine hand with the wicked to bee an vnrighteous witnesse.",
        }),

        ("exodus", 23, 2) => Some(Verse {
            content: "Thou shalt not follow a multitude to doe euill: neither shalt thou speake in a cause, to decline after many, to wrest iudgement:",
        }),

        ("exodus", 23, 3) => Some(Verse {
            content: "Neither shalt thou countenance a poore man in his cause.",
        }),

        ("exodus", 23, 4) => Some(Verse {
            content: "If thou meete thine enemies oxe or his asse going astray, thou shalt surely bring it backe to him againe.",
        }),

        ("exodus", 23, 5) => Some(Verse {
            content: "If thou see the asse of him that hateth thee, lying vnder his burden, and wouldest forbeare to helpe him, thou shalt surely helpe with him.",
        }),

        ("exodus", 23, 6) => Some(Verse {
            content: "Thou shalt not wrest the iudgement of thy poore in his cause.",
        }),

        ("exodus", 23, 7) => Some(Verse {
            content: "Keepe thee farre from a false matter: and the innocent and righteous slay thou not: for I will not iustifie the wicked.",
        }),

        ("exodus", 23, 8) => Some(Verse {
            content: "And thou shalt take no gift: for the gift blindeth the wise, and peruerteth the words of the righteous.",
        }),

        ("exodus", 23, 9) => Some(Verse {
            content: "Also thou shalt not oppresse a stranger: for yee know the heart of a stranger, seeing yee were strangers in the land of Egypt.",
        }),

        ("exodus", 23, 10) => Some(Verse {
            content: "And sixe yeres thou shalt sow thy land, and shalt gather in the fruites thereof:",
        }),

        ("exodus", 23, 11) => Some(Verse {
            content: "But the seuenth yeere thou shalt let it rest, and lie still, that the poore of thy people may eate, and what they leaue, the beasts of the field shall eate. In like maner thou shalt deale with thy vineyard, and with thy oliue yard.",
        }),

        ("exodus", 23, 12) => Some(Verse {
            content: "Sixe dayes thou shalt doe thy worke, and on the seuenth day thou shalt rest: that thine oxe and thine asse may rest, and the sonne of thy handmayd, & the stranger may be refreshed.",
        }),

        ("exodus", 23, 13) => Some(Verse {
            content: "And in all things that I haue said vnto you, be circumspect: and make no mention of the names of other gods, neither let it be heard out of thy mouth.",
        }),

        ("exodus", 23, 14) => Some(Verse {
            content: "Three times thou shalt keepe a feast vnto me in the yeere.",
        }),

        ("exodus", 23, 15) => Some(Verse {
            content: "Thou shalt keepe the feast of vnleauened bread: thou shalt eate vnleauened bread seuen daies, as I commanded thee in the time appointed of the moneth Abib: for in it thou camest out from Egypt: and none shall appeare before me emptie:",
        }),

        ("exodus", 23, 16) => Some(Verse {
            content: "And the feast of haruest, the first fruits of thy labours, which thou hast sowen in the field: and the feast of ingathering which is in the end of the yeere, when thou hast gathered in thy labours out of the field.",
        }),

        ("exodus", 23, 17) => Some(Verse {
            content: "Three times in the yeere all thy males shall appeare before the Lord God.",
        }),

        ("exodus", 23, 18) => Some(Verse {
            content: "Thou shalt not offer the blood of my sacrifice with leauened bread, neither shall the fat of my sacrifice remaine vntill the morning.",
        }),

        ("exodus", 23, 19) => Some(Verse {
            content: "The first of the first fruits of thy land thou shalt bring into the house of the Lord thy God: thou shalt not seethe a kid in his mothers milke.",
        }),

        ("exodus", 23, 20) => Some(Verse {
            content: "Behold, I send an Angel before thee to keepe thee in the way, and to bring thee into the place which I haue prepared.",
        }),

        ("exodus", 23, 21) => Some(Verse {
            content: "Beware of him, and obey his voice, prouoke him not: for he will not pardon your transgressions: for my name is in him.",
        }),

        ("exodus", 23, 22) => Some(Verse {
            content: "But if thou shalt indeed obey his voice, and doe all that I speake, then I wil be an enemie vnto thine enemies, and an aduersarie vnto thine aduersaries.",
        }),

        ("exodus", 23, 23) => Some(Verse {
            content: "For mine Angel shall goe before thee, and bring thee in vnto the Amorites, and the Hittites, and the Perizzites, and the Canaanites, the Hiuites, and the Iebusites: and I will cut them off.",
        }),

        ("exodus", 23, 24) => Some(Verse {
            content: "Thou shalt not bow downe to their gods, nor serue them, nor doe after their workes: but thou shalt vtterly ouerthrowe them, and quite breake downe their images.",
        }),

        ("exodus", 23, 25) => Some(Verse {
            content: "And yee shall serue the Lord your God, and he shall blesse thy bread, and thy water: and I will take sicknes away from the midst of thee.",
        }),

        ("exodus", 23, 26) => Some(Verse {
            content: "There shall nothing cast their yong, nor bee barren in thy land: the number of thy dayes I will fulfill.",
        }),

        ("exodus", 23, 27) => Some(Verse {
            content: "I will send my feare before thee, and will destroy all the people to whom thou shalt come, and I will make all thine enemies turne their backes vnto thee.",
        }),

        ("exodus", 23, 28) => Some(Verse {
            content: "And I will send hornets before thee, which shall driue out the Hiuite, the Canaanite, and the Hittite from before thee.",
        }),

        ("exodus", 23, 29) => Some(Verse {
            content: "I will not driue them out from before thee in one yeere, lest the land become desolate, and the beast of the field multiply against thee.",
        }),

        ("exodus", 23, 30) => Some(Verse {
            content: "By little and little I will driue them out from before thee, vntill thou be increased and inherit the land.",
        }),

        ("exodus", 23, 31) => Some(Verse {
            content: "And I will set thy bounds from the Red sea, euen vnto the sea of the Philistines, and from the desert vnto the riuer: for I will deliuer the inhabitants of the land into your hand: and thou shalt driue them out before thee.",
        }),

        ("exodus", 23, 32) => Some(Verse {
            content: "Thou shalt make no couenant with them, nor with their gods.",
        }),

        ("exodus", 23, 33) => Some(Verse {
            content: "They shall not dwell in thy land, lest they make thee sinne against me: for if thou serue their gods, it will surely be a snare vnto thee.",
        }),

        ("exodus", 24, 1) => Some(Verse {
            content: "And hee said vnto Moses, Come vp vnto þe Lord,thou, and Aaron, Nadab and Abihu, and seuentie of the Elders of Israel: and worship ye a farre off.",
        }),

        ("exodus", 24, 2) => Some(Verse {
            content: "And Moses alone shall come neere the Lord: but they shall not come nigh, neither shall the people goe vp with him.",
        }),

        ("exodus", 24, 3) => Some(Verse {
            content: "And Moses came and told the people all the words of the Lord, and all the iudgements: and all the people answered with one voyce, and said, All the words which the Lord hath said, will we doe.",
        }),

        ("exodus", 24, 4) => Some(Verse {
            content: "And Moses wrote all the words of the Lord, and rose vp early in the morning, and builded an Altar vnder the hill, and twelue pillars, according to the twelue tribes of Israel.",
        }),

        ("exodus", 24, 5) => Some(Verse {
            content: "And he sent yong men of the children of Israel, which offered burnt offerings, and sacrificed peace offerings of oxen, vnto the Lord.",
        }),

        ("exodus", 24, 6) => Some(Verse {
            content: "And Moses tooke halfe of the blood, and put it in basons, and halfe of the blood he sprinkled on the Altar.",
        }),

        ("exodus", 24, 7) => Some(Verse {
            content: "And he tooke the booke of the couenant, and read in the audience of the people: and they saide, All that the Lord hath said, will we doe, and be obedient.",
        }),

        ("exodus", 24, 8) => Some(Verse {
            content: "And Moses tooke the blood and sprinkled it on the people, and said, Behold the blood of the Couenant which the Lord hath made with you, concerning all these words.",
        }),

        ("exodus", 24, 9) => Some(Verse {
            content: "Then went vp Moses and Aaron, Nadab and Abihu, and seuenty of the Elders of Israel:",
        }),

        ("exodus", 24, 10) => Some(Verse {
            content: "And they saw the God of Israel: and there was vnder his feet, as it were a paued worke of a Saphire stone, and as it were the body of heauen in his clearenesse.",
        }),

        ("exodus", 24, 11) => Some(Verse {
            content: "And vpon the Nobles of the children of Israel he layd not his hand: also they saw God, and did eate and drinke.",
        }),

        ("exodus", 24, 12) => Some(Verse {
            content: "And the Lord sayd vnto Moses, Come vp to me into the mount, and be there, and I will giue thee Tables of stone, and a Law, and Commandements which I haue written, that thou mayest teach them.",
        }),

        ("exodus", 24, 13) => Some(Verse {
            content: "And Moses rose vp, and his minister Ioshua: and Moses went vp into the mount of God.",
        }),

        ("exodus", 24, 14) => Some(Verse {
            content: "And hee saide vnto the Elders, Tary ye here for vs, vntill wee come againe vnto you: and behold, Aaron and Hur are with you: If any man haue any matters to doe, let him come vnto them.",
        }),

        ("exodus", 24, 15) => Some(Verse {
            content: "And Moses went vp into the Mount, and a cloud couered the Mount.",
        }),

        ("exodus", 24, 16) => Some(Verse {
            content: "And the glory of the Lord abode vpon mount Sinai, and the cloud couered it sixe dayes: and the seuenth day hee called vnto Moses out of the midst of the cloud.",
        }),

        ("exodus", 24, 17) => Some(Verse {
            content: "And the sight of the glory of the Lord was like deuouring fire, on the top of the mount, in the eyes of the children of Israel.",
        }),

        ("exodus", 24, 18) => Some(Verse {
            content: "And Moses went into the midst of the cloud, and gate him vp into the mount: and Moses was in the mount forty dayes, and forty nights.",
        }),

        ("exodus", 25, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 25, 2) => Some(Verse {
            content: "Speake vnto the children of Israel, that they bring me an offering: of euery man that giueth it willingly with his heart, ye shall take my offering.",
        }),

        ("exodus", 25, 3) => Some(Verse {
            content: "And this is the offering which ye shall take of them; Gold, and siluer, and brasse,",
        }),

        ("exodus", 25, 4) => Some(Verse {
            content: "And blew, and purple, and scarlet, and fine linnen, and goats haire:",
        }),

        ("exodus", 25, 5) => Some(Verse {
            content: "And rammes skinnes died red, and badgers skinnes, and Shittim wood:",
        }),

        ("exodus", 25, 6) => Some(Verse {
            content: "Oile for the light, spices for anointing oile, and for sweet incense:",
        }),

        ("exodus", 25, 7) => Some(Verse {
            content: "Onix stones, and stones to be set in the Ephod, and in the brest plate.",
        }),

        ("exodus", 25, 8) => Some(Verse {
            content: "And let them make mee a Sanctuary, that I may dwell amongst them:",
        }),

        ("exodus", 25, 9) => Some(Verse {
            content: "According to all that I shew thee, after the patterne of the Tabernacle, and the patterne of all the instruments thereof, euen so shall ye make it.",
        }),

        ("exodus", 25, 10) => Some(Verse {
            content: "And they shall make an Arke of Shittim wood: two cubites and a halfe shalbe the length thereof, and a cubite and an halfe the breadth thereof, and a cubite & a halfe the height thereof.",
        }),

        ("exodus", 25, 11) => Some(Verse {
            content: "And thou shalt ouerlay it with pure gold, within and without shalt thou ouerlay it: and shalt make vpon it a crowne of gold round about.",
        }),

        ("exodus", 25, 12) => Some(Verse {
            content: "And thou shalt cast foure rings of gold for it, and put them in the foure corners thereof, and two rings shal be in the one side of it, and two rings in the other side of it.",
        }),

        ("exodus", 25, 13) => Some(Verse {
            content: "And thou shalt make staues of Shittim wood, and ouerlay them with gold.",
        }),

        ("exodus", 25, 14) => Some(Verse {
            content: "And thou shalt put the staues into the rings, by the sides of the Arke, that the Arke may be borne with them.",
        }),

        ("exodus", 25, 15) => Some(Verse {
            content: "The staues shall be in the rings of the Arke: they shal not be taken from it.",
        }),

        ("exodus", 25, 16) => Some(Verse {
            content: "And thou shalt put into the Arke the Testimonie which I shall giue thee.",
        }),

        ("exodus", 25, 17) => Some(Verse {
            content: "And thou shalt make a Mercieseat of pure gold: two cubites and a halfe shalbe the length thereof, and a cubite and a halfe the breadth thereof.",
        }),

        ("exodus", 25, 18) => Some(Verse {
            content: "And thou shalt make two Cherubims of gold: of beaten worke shalt thou make them, in the two endes of the Mercie-seat.",
        }),

        ("exodus", 25, 19) => Some(Verse {
            content: "And make one Cherub on the one end, and the other Cherub on the other end: euen of the Mercie-seat shall yee make the Cherubims, on the two ends thereof.",
        }),

        ("exodus", 25, 20) => Some(Verse {
            content: "And the Cherubims shall stretch forth their wings on high, couering the Mercie-seat with their wings, and their faces shall looke one to another: toward the Mercie-seat shall the faces of the Cherubims be.",
        }),

        ("exodus", 25, 21) => Some(Verse {
            content: "And thou shalt put the Mercie-seat aboue vpon the Arke, and in the Arke thou shalt put the Testimonie that I shall giue thee.",
        }),

        ("exodus", 25, 22) => Some(Verse {
            content: "And there I wil meet with thee, and I will commune with thee, from aboue the Mercie-seat, from betweene the two Cherubims which are vpon the Arke of the Testimonie, of all things which I will giue thee in commaundement vnto the children of Israel.",
        }),

        ("exodus", 25, 23) => Some(Verse {
            content: "Thou shalt also make a table of Shittim wood: two cubites shall bee the length thereof, and a cubite the bredth thereof, and a cubite and a halfe the height thereof.",
        }),

        ("exodus", 25, 24) => Some(Verse {
            content: "And thou shalt ouerlay it with pure gold, and make thereto a crowne of gold round about.",
        }),

        ("exodus", 25, 25) => Some(Verse {
            content: "And thou shalt make vnto it a border of an hand bredth round about, and thou shalt make a golden crowne to the border thereof round about.",
        }),

        ("exodus", 25, 26) => Some(Verse {
            content: "And thou shalt make for it foure rings of gold, and put the rings in the foure corners that are on the foure feete thereof.",
        }),

        ("exodus", 25, 27) => Some(Verse {
            content: "Ouer against the border shall the rings be for places of the staues to beare the table.",
        }),

        ("exodus", 25, 28) => Some(Verse {
            content: "And thou shalt make the staues of Shittim wood, and ouerlay them with gold, that the table may be borne with them.",
        }),

        ("exodus", 25, 29) => Some(Verse {
            content: "And thou shalt make the dishes thereof, and spoones therof, and couers thereof, and bowles thereof, to couer withall: of pure gold shalt thou make them.",
        }),

        ("exodus", 25, 30) => Some(Verse {
            content: "And thou shalt set vpon the Table Shew-bread before me alway.",
        }),

        ("exodus", 25, 31) => Some(Verse {
            content: "And thou shalt make a Candlesticke of pure gold: of beaten worke shall the candlesticke bee made; his shaft and his branches, his bowles, his knops, and his flowers shall be of the same.",
        }),

        ("exodus", 25, 32) => Some(Verse {
            content: "And sixe branches shall come out of the sides of it: three branches of the candlesticke out of the one side, and three branches of the candlesticke out of the other side:",
        }),

        ("exodus", 25, 33) => Some(Verse {
            content: "Three bowles made like vnto almonds, with a knop and a flower in one branch: and three bowles made like almonds in the other branch, with a knop and a flower: so in the sixe branches that come out of the candlesticke.",
        }),

        ("exodus", 25, 34) => Some(Verse {
            content: "And in the candlesticke shall bee foure bowles made like vnto almonds, with their knops and their flowers.",
        }),

        ("exodus", 25, 35) => Some(Verse {
            content: "And there shal be a knop vnder two branches of the same, and a knop vnder two branches of the same, and a knop vnder two branches of the same, according to the sixe branches that proceede out of the candlesticke.",
        }),

        ("exodus", 25, 36) => Some(Verse {
            content: "Their knops and their branches shall be of the same: all it shall bee one beaten worke of pure gold.",
        }),

        ("exodus", 25, 37) => Some(Verse {
            content: "And thou shalt make the seuen lamps thereof: and they shall light the lampes thereof, that they may giue light ouer against it.",
        }),

        ("exodus", 25, 38) => Some(Verse {
            content: "And the tongs thereof, and the snuffe dishes therof shalbe of pure gold.",
        }),

        ("exodus", 25, 39) => Some(Verse {
            content: "Of a talent of pure gold shall hee make it, with all these vessels.",
        }),

        ("exodus", 25, 40) => Some(Verse {
            content: "And looke that thou make them after their patterne, which was shewed thee in the mount.",
        }),

        ("exodus", 26, 1) => Some(Verse {
            content: "Moreouer thou shalt make the Tabernacle with ten curtaines of fine twined linnen, and blew, and purple, and scarlet: with Cherubims of cunning worke shalt thou make them.",
        }),

        ("exodus", 26, 2) => Some(Verse {
            content: "The length of one curtaine shalbe eight and twenty cubits, and the bredth of one curtaine, foure cubits: and euery one of the curtaines shall haue one measure.",
        }),

        ("exodus", 26, 3) => Some(Verse {
            content: "The fiue curtaines shalbe coupled together one to another: and other fiue curtaines shalbe coupled one to another.",
        }),

        ("exodus", 26, 4) => Some(Verse {
            content: "And thou shalt make loopes of blew vpon the edge of the one curtaine, from the seluedge in the coupling, and likewise shalt thou make in the vttermost edge of another curtaine, in the coupling of the second.",
        }),

        ("exodus", 26, 5) => Some(Verse {
            content: "Fiftie loopes shalt thou make in the one curtaine, and fiftie loopes shalt thou make in the edge of the curtaine, that is in the coupling of the second, that the loopes may take hold one of another.",
        }),

        ("exodus", 26, 6) => Some(Verse {
            content: "And thou shalt make fiftie taches of gold, and couple the curtaines together with the taches: and it shall be one tabernacle.",
        }),

        ("exodus", 26, 7) => Some(Verse {
            content: "And thou shalt make curtaines of goats haire, to be a couering vpon the tabernacle: eleuen curtaines shalt thou make.",
        }),

        ("exodus", 26, 8) => Some(Verse {
            content: "The length of one curtaine shalbe thirtie cubites, and the bredth of one curtaine foure cubites: and the eleuen shalbe all of one measure.",
        }),

        ("exodus", 26, 9) => Some(Verse {
            content: "And thou shalt couple fiue curtaines by themselues, and sixe curtaines by themselues, and shalt double the sixt curtaine in the forefront of the tabernacle.",
        }),

        ("exodus", 26, 10) => Some(Verse {
            content: "And thou shalt make fiftie loopes on the edge of the one curtaine, that is outmost in the coupling, and fiftie loopes in the edge of the curtaine which coupleth the second.",
        }),

        ("exodus", 26, 11) => Some(Verse {
            content: "And thou shalt make fiftie taches of brasse, and put the taches into the loopes, and couple the tent together, that it may be one.",
        }),

        ("exodus", 26, 12) => Some(Verse {
            content: "And the remnant that remaineth of the curtaines of the tent, the halfe curtaine that remaineth shall hang ouer the backe side of the tabernacle.",
        }),

        ("exodus", 26, 13) => Some(Verse {
            content: "And a cubite on the one side, and a cubite on the other side of that which remaineth in the length of the curtaines of the tent, it shall hang ouer the sides of the tabernacle, on this side, and on that side to couer it.",
        }),

        ("exodus", 26, 14) => Some(Verse {
            content: "And thou shalt make a couering for the tent of rammes skinnes died red, and a couering aboue of badgers skinnes.",
        }),

        ("exodus", 26, 15) => Some(Verse {
            content: "And thou shalt make boards for the Tabernacle of Shittim wood standing vp.",
        }),

        ("exodus", 26, 16) => Some(Verse {
            content: "Ten cubits shall be the length of a board, and a cubite and an halfe shall be the breadth of one board.",
        }),

        ("exodus", 26, 17) => Some(Verse {
            content: "Two tenons shall there be in one board set in order one against another: thus shalt thou make for all the boards of the Tabernacle.",
        }),

        ("exodus", 26, 18) => Some(Verse {
            content: "And thou shalt make the boards for the Tabernacle, twentie boards on the Southside Southward.",
        }),

        ("exodus", 26, 19) => Some(Verse {
            content: "And thou shalt make fourtie sockets of siluer, vnder the twenty boards: two sockets vnder one board for his two tenons, and two sockets vnder another board for his two tenons.",
        }),

        ("exodus", 26, 20) => Some(Verse {
            content: "And for the second side of the Tabernacle on the Northside there shall bee twentie boards,",
        }),

        ("exodus", 26, 21) => Some(Verse {
            content: "And their fourtie sockets of siluer: two sockets vnder one board, and two sockets vnder another board.",
        }),

        ("exodus", 26, 22) => Some(Verse {
            content: "And for the sides of the Tabernacle Westward thou shalt make sixe boards.",
        }),

        ("exodus", 26, 23) => Some(Verse {
            content: "And two boards shalt thou make for the corners of the tabernacle in the two sides.",
        }),

        ("exodus", 26, 24) => Some(Verse {
            content: "And they shall be coupled together beneath, and they shall be coupled together aboue the head of it vnto one ring: thus shall it bee for them both; they shall be for the two corners.",
        }),

        ("exodus", 26, 25) => Some(Verse {
            content: "And they shall be eight boards, and their sockets of siluer sixteene sockets: two sockets vnder one board, and two sockets vnder another board.",
        }),

        ("exodus", 26, 26) => Some(Verse {
            content: "And thou shalt make barres of Shittim wood: fiue for the boards of the one side of the Tabernacle,",
        }),

        ("exodus", 26, 27) => Some(Verse {
            content: "And fiue barres for the boards of the other side of the Tabernacle, and fiue barres for the boards of the side of the Tabernacle for the two sides Westward.",
        }),

        ("exodus", 26, 28) => Some(Verse {
            content: "And the middle barre in the mids of the boards, shall reach from ende to ende.",
        }),

        ("exodus", 26, 29) => Some(Verse {
            content: "And thou shalt ouerlay the boards with gold, and make their rings of gold for places for the barres: and thou shalt ouerlay the barres with gold.",
        }),

        ("exodus", 26, 30) => Some(Verse {
            content: "And thou shalt reare vp the Tabernacle according to the fashion therof, which was shewed thee in the mount.",
        }),

        ("exodus", 26, 31) => Some(Verse {
            content: "And thou shalt make a Uaile of blew, and purple, and scarlet, and fine twined linnen of cunning worke: with Cherubims shall it be made.",
        }),

        ("exodus", 26, 32) => Some(Verse {
            content: "And thou shalt hang it vpon foure pillars of Shittim wood, ouerlayd with gold: their hookes shalbe of gold, vpon the foure sockets of siluer.",
        }),

        ("exodus", 26, 33) => Some(Verse {
            content: "And thou shalt hang vp the Uaile vnder the taches, that thou maist bring in thither within the Uaile, the Arke of the Testimony: and the Uaile shall diuide vnto you, betweene the holy place and the most holy.",
        }),

        ("exodus", 26, 34) => Some(Verse {
            content: "And thou shalt put the Mercie-seat vpon the Arke of the Testimony, in the most holy place.",
        }),

        ("exodus", 26, 35) => Some(Verse {
            content: "And thou shalt set the table without the Uaile, and the candlesticke ouer against the table, on the side of the Tabernacle toward the South: and thou shalt put the table on the North side.",
        }),

        ("exodus", 26, 36) => Some(Verse {
            content: "And thou shalt make an Hanging for the doore of the Tent, of blew, and purple and scarlet, and fine twined linnen, wrought with needle worke.",
        }),

        ("exodus", 26, 37) => Some(Verse {
            content: "And thou shalt make for the Hanging fiue pillars of Shittim wood, and ouerlay them with gold, and their hookes shalbe of gold: and thou shalt cast fiue sockets of brasse for them.",
        }),

        ("exodus", 27, 1) => Some(Verse {
            content: "And thou shalt make an Altar of Shittim wood, fiue cubits long, and fiue cubites broad: the Altar shall be foure square, and the height thereof shalbe three cubits.",
        }),

        ("exodus", 27, 2) => Some(Verse {
            content: "And thou shalt make the hornes of it vpon the foure corners thereof: his hornes shall be of the same: and thou shalt ouerlay it with brasse.",
        }),

        ("exodus", 27, 3) => Some(Verse {
            content: "And thou shalt make his pannes to receiue his ashes, and his shouels, and his basons, and his fleshhooks, and his firepannes: all the vessels thereof thou shalt make of brasse.",
        }),

        ("exodus", 27, 4) => Some(Verse {
            content: "And thou shalt make for it a grate of networke of brasse; and vpon the net shalt thou make foure brasen rings in the foure corners thereof.",
        }),

        ("exodus", 27, 5) => Some(Verse {
            content: "And thou shalt put it vnder the compasse of the Altar beneath, that the net may bee euen to the midst of the Altar.",
        }),

        ("exodus", 27, 6) => Some(Verse {
            content: "And thou shalt make staues for the Altar, staues of Shittim wood, and ouerlay them with brasse.",
        }),

        ("exodus", 27, 7) => Some(Verse {
            content: "And the staues shalbe put into the rings, and the staues shall be vpon the two sides of the Altar, to beare it.",
        }),

        ("exodus", 27, 8) => Some(Verse {
            content: "Hollow with boards shalt thou make it: as it was shewed thee in the mount, so shall they make it.",
        }),

        ("exodus", 27, 9) => Some(Verse {
            content: "And thou shalt make the Court of the Tabernacle for the Southside, Southward: there shall be hangings for the Court, of fine twined linnen of an hundred cubits long, for one side.",
        }),

        ("exodus", 27, 10) => Some(Verse {
            content: "And the twenty pillars thereof, and their twenty sockets, shall be of brasse: the hookes of the pillars, and their fillets shalbe of siluer.",
        }),

        ("exodus", 27, 11) => Some(Verse {
            content: "And likewise for the Northside in length, there shall be hangings of an hundred cubits long, and his twenty pillars, and their twenty sockets of brasse: the hookes of the pillars, and their fillets of siluer.",
        }),

        ("exodus", 27, 12) => Some(Verse {
            content: "And for the breadth of the Court, on the Westside shalbe hangings of fifty cubits: their pillars tenne, and their sockets ten.",
        }),

        ("exodus", 27, 13) => Some(Verse {
            content: "And the breadth of the Court on the Eastside Eastward, shall bee fiftie cubits.",
        }),

        ("exodus", 27, 14) => Some(Verse {
            content: "The hangings of one side of the gate shalbe fifteene cubits: their pillars three, and their sockets three.",
        }),

        ("exodus", 27, 15) => Some(Verse {
            content: "And on the other side shalbe hangings, fifteene cubits: their pillars three, and their sockets three.",
        }),

        ("exodus", 27, 16) => Some(Verse {
            content: "And for the gate of the Court shall be an hanging of twenty cubits of blew, and purple, and scarlet, and fine twined linnen, wrought with needle worke: and their pillars shall be foure, and their sockets foure.",
        }),

        ("exodus", 27, 17) => Some(Verse {
            content: "All the pillars round about the Court shalbe filletted with siluer: their hookes shalbe of siluer, and their sockets of brasse.",
        }),

        ("exodus", 27, 18) => Some(Verse {
            content: "The length of the Court shalbe an hundred cubits, and the breadth fiftie euery where, and the height fiue cubits of fine twined linnen, and their sockets of brasse.",
        }),

        ("exodus", 27, 19) => Some(Verse {
            content: "All the vessels of the Tabernacle in all the seruice thereof, and all the pinnes thereof, and all the pinnes of the Court, shalbe of brasse.",
        }),

        ("exodus", 27, 20) => Some(Verse {
            content: "And thou shalt command the children of Israel, that they bring thee pure oyle Oliue beaten, for the light, to cause the lampe to burne alwayes.",
        }),

        ("exodus", 27, 21) => Some(Verse {
            content: "In the Tabernacle of the Congregation without the Uaile, which is before the Testimony, Aaron and his sonnes shall order it from euening to morning before the Lord: It shall be a statute for euer, vnto their generations, on the behalfe of the children of Israel.",
        }),

        ("exodus", 28, 1) => Some(Verse {
            content: "And take thou vnto thee Aaron thy brother, and his sonnes with him, from among the children of Israel, that he may minister vnto me in the Priests office, euen Aaron, Nadab, and Abihu, Eleazar, and Ithamar, Aarons sonnes.",
        }),

        ("exodus", 28, 2) => Some(Verse {
            content: "And thou shalt make holy garments for Aaron thy brother, for glory and for beauty.",
        }),

        ("exodus", 28, 3) => Some(Verse {
            content: "And thou shalt speake vnto all that are wise hearted, whom I haue filled with the spirit of wisedome, that they may make Aarons garments to consecrate him, that hee may minister vnto me in the Priests office.",
        }),

        ("exodus", 28, 4) => Some(Verse {
            content: "And these are the garments which they shall make; a breastplate, and an Ephod, and a robe, and a broidered coat, a Miter, and a girdle: and they shall make holy garments for Aaron thy brother, and his sonnes, that hee may minister vnto mee in the Priestes office.",
        }),

        ("exodus", 28, 5) => Some(Verse {
            content: "And they shall take gold, and blew, and purple, and scarlet, and fine linnen.",
        }),

        ("exodus", 28, 6) => Some(Verse {
            content: "And they shall make the Ephod of gold, of blew and of purple, of scarlet, and fine twined linnen, with cunning worke.",
        }),

        ("exodus", 28, 7) => Some(Verse {
            content: "It shall haue the two shoulder pieces thereof, ioyned at the two edges thereof; and so it shall bee ioyned together.",
        }),

        ("exodus", 28, 8) => Some(Verse {
            content: "And the curious girdle of the Ephod which is vpon it, shall bee of the same, according to the worke thereof, euen of gold, of blew, and purple, and scarlet, and fine twined linnen.",
        }),

        ("exodus", 28, 9) => Some(Verse {
            content: "And thou shalt take two Onix stones, and graue on them the names of the children of Israel:",
        }),

        ("exodus", 28, 10) => Some(Verse {
            content: "Sixe of their names on one stone, and the other sixe names of the rest on the other stone, according to their birth:",
        }),

        ("exodus", 28, 11) => Some(Verse {
            content: "With the worke of an engrauer in stone; like the engrauings of a signet shalt thou engraue the two stones, with the names of the children of Israel; thou shalt make them to be set in ouches of gold.",
        }),

        ("exodus", 28, 12) => Some(Verse {
            content: "And thou shalt put the two stones vpon the shoulders of the Ephod, for stones of memoriall vnto the children of Israel. And Aaron shall beare their names before the Lord, vpon his two shoulders for a memoriall.",
        }),

        ("exodus", 28, 13) => Some(Verse {
            content: "And thou shalt make ouches of gold;",
        }),

        ("exodus", 28, 14) => Some(Verse {
            content: "And two chaines of pure gold at the ends; of wreathen worke shalt thou make them, and fasten the wreathen chaines to the ouches.",
        }),

        ("exodus", 28, 15) => Some(Verse {
            content: "And thou shalt make the brestplate of Iudgement, with cunning worke, after the worke of the Ephod thou shalt make it: of gold, of blew, and of purple, and of scarlet, and of fine twined linnen shalt thou make it.",
        }),

        ("exodus", 28, 16) => Some(Verse {
            content: "Foure square it shall be being doubled; a spanne shalbe the length thereof, and a span shalbe the breadth thereof.",
        }),

        ("exodus", 28, 17) => Some(Verse {
            content: "And thou shalt set in it settings of stones; euen foure rowes of stones: the first row shalbe a Sardius, a Topaz, and a Carbuncle: this shall be the first row.",
        }),

        ("exodus", 28, 18) => Some(Verse {
            content: "And the second row shall be an Emeraude, a Saphir, and a Diamond.",
        }),

        ("exodus", 28, 19) => Some(Verse {
            content: "And the third row a Lygure, an Agate, and an Amethist.",
        }),

        ("exodus", 28, 20) => Some(Verse {
            content: "And the fourth row, a Berill, and an Onix, and a Iasper: they shalbe set in gold in their inclosings.",
        }),

        ("exodus", 28, 21) => Some(Verse {
            content: "And the stones shall bee with the names of the children of Israel, twelue, according to their names, like the engrauings of a signet: euery one with his name shall they bee according to the twelue tribes.",
        }),

        ("exodus", 28, 22) => Some(Verse {
            content: "And thou shalt make vpon the brestplate chaines at the ends, of wreathen worke, of pure gold.",
        }),

        ("exodus", 28, 23) => Some(Verse {
            content: "And thou shalt make vpon the brestplate two rings of gold, and shalt put the two rings on the two endes of the brestplate.",
        }),

        ("exodus", 28, 24) => Some(Verse {
            content: "And thou shalt put the two wreathen chaines of gold in the two rings, which are on the ends of the brestplate.",
        }),

        ("exodus", 28, 25) => Some(Verse {
            content: "And the other two endes of the two wreathen chaines, thou shalt fasten in the two ouches, and put them on the shoulder pieces of the Ephod before it.",
        }),

        ("exodus", 28, 26) => Some(Verse {
            content: "And thou shalt make two rings of gold, and thou shalt put them vpon the two ends of the breastplate, in the border thereof, which is in the side of the Ephod inward.",
        }),

        ("exodus", 28, 27) => Some(Verse {
            content: "And two other rings of gold thou shalt make, and shalt put them on the two sides of the Ephod vnderneath towards the forepart thereof, ouer against the other coupling thereof, aboue the curious girdle of the Ephod.",
        }),

        ("exodus", 28, 28) => Some(Verse {
            content: "And they shall bind the brestplate by the rings thereof, vnto the rings of the Ephod with a lace of blewe, that it may be aboue the curious girdle of the Ephod, and that the breastplate be not loosed from the Ephod.",
        }),

        ("exodus", 28, 29) => Some(Verse {
            content: "And Aaron shal beare the names of the children of Israel in the breastplate of iudgement, vpon his heart, when hee goeth in vnto the holy place, for a memoriall before the Lord continually.",
        }),

        ("exodus", 28, 30) => Some(Verse {
            content: "And thou shalt put in the breastplate of iudgement, the Urim and the Thummim, and they shall bee vpon Aarons heart, when he goeth in before the Lord: and Aaron shall beare the iudgement of the children of Israel vpon his heart, before the Lord continually.",
        }),

        ("exodus", 28, 31) => Some(Verse {
            content: "And thou shalt make the robe of the Ephod all of blew.",
        }),

        ("exodus", 28, 32) => Some(Verse {
            content: "And there shall bee an hole in the top of it, in the mids thereof: it shall haue a binding of wouen worke, round about the hole of it, as it were the hole of an habergeon, that it be not rent.",
        }),

        ("exodus", 28, 33) => Some(Verse {
            content: "And beneath vpon the hemme of it thou shalt make pomegranates of blew, and of purple, and of scarlet, round about the hemme thereof, and belles of gold betweene them round about.",
        }),

        ("exodus", 28, 34) => Some(Verse {
            content: "A golden bell and a pomegranate, a golden bell and a pomegranate, vpon the hemme of the robe round about.",
        }),

        ("exodus", 28, 35) => Some(Verse {
            content: "And it shall be vpon Aaron, to minister: and his sound shall be heard when he goeth in vnto the holy place before the Lord, and when he commeth out, that he die not.",
        }),

        ("exodus", 28, 36) => Some(Verse {
            content: "And thou shalt make a plate of pure gold, and graue vpon it, like the engrauings of a signet, HOLINES TO THE LORD.",
        }),

        ("exodus", 28, 37) => Some(Verse {
            content: "And thou shalt put it on a blewe lace, that it may be vpon the miter; vpon the forefront of the miter it shall be.",
        }),

        ("exodus", 28, 38) => Some(Verse {
            content: "And it shall be vpon Aarons forehead, that Aaron may beare the iniquitie of the holy things, which the children of Israel shall hallow, in all their holy gifts: and it shall be alwayes vpon his forehead, that they may be accepted before the Lord.",
        }),

        ("exodus", 28, 39) => Some(Verse {
            content: "And thou shalt embroider the coat of fine linnen, and thou shalt make the miter of fine linnen, and thou shalt make the girdle of needle worke.",
        }),

        ("exodus", 28, 40) => Some(Verse {
            content: "And for Aarons sonnes thou shalt make coats, and thou shalt make for them girdles, and bonnets shalt thou make for them, for glory and for beautie.",
        }),

        ("exodus", 28, 41) => Some(Verse {
            content: "And thou shalt put them vpon Aaron thy brother, and his sonnes with him: and shalt annoint them, and consecrate them, and sanctifie them, that they may minister vnto mee in the Priests office.",
        }),

        ("exodus", 28, 42) => Some(Verse {
            content: "And thou shalt make them linnen breeches, to couer their nakednes, from the loines euen vnto the thighes they shall reach.",
        }),

        ("exodus", 28, 43) => Some(Verse {
            content: "And they shall be vpon Aaron, & vpon his sonnes, when they come in vnto the Tabernacle of the Congregation, or when they come neere vnto the Altar to minister in the holy place, that they beare not iniquitie, and die. It shall be a statute for euer vnto him and his seede after him.",
        }),

        ("exodus", 29, 1) => Some(Verse {
            content: "And this is the thing that thou shalt doe vnto them, to hallow them, to minister vnto me in the Priests office: Take one yong bullocke, and two rammes without blemish,",
        }),

        ("exodus", 29, 2) => Some(Verse {
            content: "And vnleauened bread, and cakes unleauened, tempered with oyle, and wafers vnleauened, annointed with oile: of wheaten flowre shalt thou make them.",
        }),

        ("exodus", 29, 3) => Some(Verse {
            content: "And thou shalt put them into one basket, and bring them in the basket, with the bullocke and the two rammes.",
        }),

        ("exodus", 29, 4) => Some(Verse {
            content: "And Aaron and his sonnes thou shalt bring vnto the doore of the Tabernacle of the Congregation, and shalt wash them with water.",
        }),

        ("exodus", 29, 5) => Some(Verse {
            content: "And thou shalt take the garments, and put vpon Aaron the coat, and the robe of the Ephod, and the Ephod, and the brestplate, and gird him with the curious girdle of the Ephod.",
        }),

        ("exodus", 29, 6) => Some(Verse {
            content: "And thou shalt put the Miter vpon his head, and put the holy Crowne vpon the Miter.",
        }),

        ("exodus", 29, 7) => Some(Verse {
            content: "Then shalt thou take the annointing oyle, and powre it vpon his head, and annoint him.",
        }),

        ("exodus", 29, 8) => Some(Verse {
            content: "And thou shalt bring his sonnes, and put coats vpon them.",
        }),

        ("exodus", 29, 9) => Some(Verse {
            content: "And thou shalt gird them with girdles, (Aaron and his sonnes) and put the bonnets on them: and the priests office shall be theirs for a perpetuall statute: and thou shalt consecrate Aaron and his sonnes.",
        }),

        ("exodus", 29, 10) => Some(Verse {
            content: "And thou shalt cause a bullocke to bee brought before the Tabernacle of the Congregation: and Aaron and his sonnes shall put their hands vpon the head of the bullocke.",
        }),

        ("exodus", 29, 11) => Some(Verse {
            content: "And thou shalt kill the bullocke before the Lord, by the doore of the Tabernacle of the Congregation.",
        }),

        ("exodus", 29, 12) => Some(Verse {
            content: "And thou shalt take of the blood of the bullocke, and put it vpon the hornes of the altar with thy finger, and powre all the blood beside the bottome of the Altar.",
        }),

        ("exodus", 29, 13) => Some(Verse {
            content: "And thou shalt take all the fat that couereth the inwards, and the caule that is aboue the liuer, and the two kidneis, and the fat that is vpon them, and burne them vpon the altar.",
        }),

        ("exodus", 29, 14) => Some(Verse {
            content: "But the flesh of the bullocke, and his skinne, and his doung shalt thou burne with fire without the campe, it is a sinne offering.",
        }),

        ("exodus", 29, 15) => Some(Verse {
            content: "Thou shalt also take one ram, and Aaron and his sonnes shall put their hands vpon the head of the ram.",
        }),

        ("exodus", 29, 16) => Some(Verse {
            content: "And thou shalt slay the ramme, and thou shalt take his blood, and sprinkle it round about vpon the altar.",
        }),

        ("exodus", 29, 17) => Some(Verse {
            content: "And thou shalt cut the ramme in pieces, and wash the inwards of him, and his legs, and put them vnto his pieces, and vnto his head.",
        }),

        ("exodus", 29, 18) => Some(Verse {
            content: "And thou shalt burne the whole ramme vpon the Altar: it is a burnt offering vnto the Lord: It is a sweet sauour, an offering made by fire vnto the Lord.",
        }),

        ("exodus", 29, 19) => Some(Verse {
            content: "And thou shalt take the other ramme: and Aaron and his sonnes shall put their hands vpon the head of the ramme.",
        }),

        ("exodus", 29, 20) => Some(Verse {
            content: "Then shalt thou kill the ramme, and take of his blood, and put it vpon the tip of the right eare of Aaron, and vpon the tip of the right eare of his sonnes, and vpon the thumbe of their right hand, and vpon the great toe of their right foot, and sprinckle the blood vpon the Altar round about.",
        }),

        ("exodus", 29, 21) => Some(Verse {
            content: "And thou shalt take of the blood that is vpon the Altar, and of the anointing oyle, and sprinkle it vpon Aaron, and vpon his garments, and vpon his sonnes, and vpon the garments of his sonnes with him: and hee shall be hallowed, and his garments, and his sonnes, and his sonnes garments with him.",
        }),

        ("exodus", 29, 22) => Some(Verse {
            content: "Also thou shalt take of the ram the fat and the rumpe, and the fat that couereth the inwards, & the caule aboue the liuer, and the two kidneis, and the fat that is vpon them, and the right shoulder, for it is a ram of consecration:",
        }),

        ("exodus", 29, 23) => Some(Verse {
            content: "And one loafe of bread, and one cake of oyled bread, and one wafer out of the basket of the vnleauened bread, that is before the Lord.",
        }),

        ("exodus", 29, 24) => Some(Verse {
            content: "And thou shalt put all in the hands of Aaron, and in the hands of his sonnes, and shalt waue them for a waue-offering before the Lord.",
        }),

        ("exodus", 29, 25) => Some(Verse {
            content: "And thou shalt receiue them of their hands, and burne them vpon the Altar for a burnt offering, for a sweet sauour before the Lord: it is an offering made by fire vnto the Lord.",
        }),

        ("exodus", 29, 26) => Some(Verse {
            content: "And thou shalt take the brest of the ramme of Aarons consecrations, and waue it for a waue-offering before the Lord, and it shalbe thy part.",
        }),

        ("exodus", 29, 27) => Some(Verse {
            content: "And thou shalt sanctifie the brest of the waue-offering, and the shoulder of the heaue offering, which is waued, and which is heaued vp of the ramme of the consecration, euen of that which is for Aaron, and of that which is for his sonnes.",
        }),

        ("exodus", 29, 28) => Some(Verse {
            content: "And it shalbe Aarons, and his sonnes by a statute for euer, from the children of Israel: for it is an heaue offering: and it shall be an heaue offering from the children of Israel, of the sacrifice of their peace offrings, euen their heaue offering vnto the Lord.",
        }),

        ("exodus", 29, 29) => Some(Verse {
            content: "And the holy garments of Aaron shall be his sonnes after him, to bee anoynted therein, and to be consecrated in them.",
        }),

        ("exodus", 29, 30) => Some(Verse {
            content: "And that sonne that is Priest in his stead, shal put them on seuen dayes, when he commeth into the Tabernacle of the Congregation to minister in the holy place.",
        }),

        ("exodus", 29, 31) => Some(Verse {
            content: "And thou shalt take the ramme of the consecration, and seethe his flesh in the holy place.",
        }),

        ("exodus", 29, 32) => Some(Verse {
            content: "And Aaron and his sonnes shall eate the flesh of the ramme, and the bread that is in the basket, by the doore of the Tabernacle of the Cōgregation.",
        }),

        ("exodus", 29, 33) => Some(Verse {
            content: "And they shall eate those things, wherewith the atonement was made, to consecrate and to sanctifie them: but a stranger shall not eate thereof, because they are holy.",
        }),

        ("exodus", 29, 34) => Some(Verse {
            content: "And if ought of the flesh of the consecrations, or of the bread remaine vnto the morning, then thou shalt burne the remainder with fire: it shall not be eaten, because it is holy.",
        }),

        ("exodus", 29, 35) => Some(Verse {
            content: "And thus shalt thou doe vnto Aaron, and to his sonnes, according to all things which I haue commaunded thee: seuen dayes shalt thou consecrate them.",
        }),

        ("exodus", 29, 36) => Some(Verse {
            content: "And thou shalt offer euery day a bullocke for a sinne offering, for atonement: and thou shalt clense the Altar, when thou hast made an atonement for it, and thou shalt anoynt it, to sanctifie it.",
        }),

        ("exodus", 29, 37) => Some(Verse {
            content: "Seuen dayes thou shalt make an atonement for the Altar, and sanctifie it: and it shalbe an Altar most holy: whatsoeuer toucheth the Altar, shalbe holy.",
        }),

        ("exodus", 29, 38) => Some(Verse {
            content: "Now this is that which thou shalt offer vpon the Altar; two lambs of the first yere, day by day continually.",
        }),

        ("exodus", 29, 39) => Some(Verse {
            content: "The one lambe thou shalt offer in the morning: and the other lambe thou shalt offer at euen:",
        }),

        ("exodus", 29, 40) => Some(Verse {
            content: "And with the one lambe a tenth deale of flowre mingled with the fourth part of an Hin of beaten oyle: and the fourth part of an Hin of wine for a drinke offering.",
        }),

        ("exodus", 29, 41) => Some(Verse {
            content: "And the other lambe thou shalt offer at Euen, and shalt doe thereto, according to the meat offering of the morning, and according to the drinke offering thereof, for a sweet sauour, an offering made by fire vnto the Lord.",
        }),

        ("exodus", 29, 42) => Some(Verse {
            content: "This shalbe a continuall burnt offering throughout your generations, at the doore of the Tabernacle of the Congregation, before the Lord,where I wil meete you, to speake there vnto thee.",
        }),

        ("exodus", 29, 43) => Some(Verse {
            content: "And there I will meet with the children of Israel: and the Tabernacle shalbe sanctified by my glory.",
        }),

        ("exodus", 29, 44) => Some(Verse {
            content: "And I will sanctifie the Tabernacle of the Congregation, and the Altar: I will sanctifie also both Aaron and his sonnes, to minister to me in the Priests office.",
        }),

        ("exodus", 29, 45) => Some(Verse {
            content: "And I will dwell amongst the children of Israel, and will be their God.",
        }),

        ("exodus", 29, 46) => Some(Verse {
            content: "And they shall know that I am the Lord their God, that brought them foorth out of the land of Egypt, that I may dwell amongst them: I am the Lord their God.",
        }),

        ("exodus", 30, 1) => Some(Verse {
            content: "Andthou shalt make an Altar to burne incense vpon: of Shittim wood shalt thou make it.",
        }),

        ("exodus", 30, 2) => Some(Verse {
            content: "A cubite shall bee the length thereof, and a cubite the breadth thereof, (foure square shall it bee) and two cubits shalbe the height thereof: the hornes thereof shalbe of the same.",
        }),

        ("exodus", 30, 3) => Some(Verse {
            content: "And thou shalt ouerlay it with pure gold, the top therof, and the sides thereof round about, and the hornes thereof: and thou shalt make vnto it a crowne of gold round about.",
        }),

        ("exodus", 30, 4) => Some(Verse {
            content: "And two golden rings shalt thou make to it vnder the crowne of it, by the two corners thereof, vpon the two sides of it shalt thou make it: and they shalbe for places for the staues to beare it withall.",
        }),

        ("exodus", 30, 5) => Some(Verse {
            content: "And thou shalt make the staues of Shittim wood, and ouerlay them with gold.",
        }),

        ("exodus", 30, 6) => Some(Verse {
            content: "And thou shalt put it before the Uaile, that is by the Arke of the Testimonie before the Mercie-seat, that is, ouer the Testimonie where I will meet with thee.",
        }),

        ("exodus", 30, 7) => Some(Verse {
            content: "And Aaron shall burne thereon sweet incense euery morning: when he dresseth the lamps he shal burne incense vpon it.",
        }),

        ("exodus", 30, 8) => Some(Verse {
            content: "And when Aaron lighteth the lampes at euen, he shall burne incense vpon it, a perpetuall incense before the Lord,throughout your generations.",
        }),

        ("exodus", 30, 9) => Some(Verse {
            content: "Ye shall offer no strange incense thereon, nor burnt sacrifice, nor meate offering, neither shall ye powre drinke offering thereon.",
        }),

        ("exodus", 30, 10) => Some(Verse {
            content: "And Aaron shall make an atonement vpon the hornes of it once in a yeere, with the blood of the sinne offering of atonements: once in the yeere shall hee make atonement vpon it, throughout your generations: it is most holy vnto the Lord.",
        }),

        ("exodus", 30, 11) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 30, 12) => Some(Verse {
            content: "When thou takest the summe of the children of Israel, after their number, then shall they giue euery man a ransome for his soule vnto the Lord, when thou numbrest them, that there be no plague amongst them, when thou numbrest them.",
        }),

        ("exodus", 30, 13) => Some(Verse {
            content: "This they shall giue, euery one that passeth among them that are numbred: halfe a shekel after the shekel of the Sanctuary: A shekel is twenty gerahs: an halfe shekel shall be the offering of the Lord.",
        }),

        ("exodus", 30, 14) => Some(Verse {
            content: "Euery one that passeth among them that are numbred from twentie yeeres old and aboue, shall giue an offering vnto the Lord.",
        }),

        ("exodus", 30, 15) => Some(Verse {
            content: "The rich shal not giue more, and the poore shall not giue lesse then halfe a shekel, when they giue an offering vnto the Lord, to make an atonement for your soules.",
        }),

        ("exodus", 30, 16) => Some(Verse {
            content: "And thou shalt take the atonement money of the children of Israel, and shalt appoint it for the seruice of the Tabernacle of the Congregation, that it may be a memoriall vnto the children of Israel before the Lord, to make an atonement for your soules.",
        }),

        ("exodus", 30, 17) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 30, 18) => Some(Verse {
            content: "Thou shalt also make a Lauer of brasse, and his foote also of brasse, to wash withall, and thou shalt put it betweene the Tabernacle of the Congregation, and the altar, and thou shalt put water therein.",
        }),

        ("exodus", 30, 19) => Some(Verse {
            content: "For Aaron and his sonnes shall wash their hands and their feet thereat.",
        }),

        ("exodus", 30, 20) => Some(Verse {
            content: "When they goe into the Tabernacle of the Congregation, they shall wash with water, that they die not: or when they come neere to the altar to minister, to burne offering made by fire vnto the Lord.",
        }),

        ("exodus", 30, 21) => Some(Verse {
            content: "So they shall wash their handes and their feet, that they die not: and it shall be a statute for euer to them, euen to him and to his seed throughout their generations.",
        }),

        ("exodus", 30, 22) => Some(Verse {
            content: "Moreouer the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 30, 23) => Some(Verse {
            content: "Take thou also vnto thee principall spices, of pure myrrhe fiue hundred shekels, and of sweet cinamon halfe so much, euen two hundred and fifty shekels, and of sweet calamus two hundred and fiftie shekels,",
        }),

        ("exodus", 30, 24) => Some(Verse {
            content: "And of Cassia fiue hundred shekels, after the shekel of the Sanctuary, and of oyle oliue an Hin.",
        }),

        ("exodus", 30, 25) => Some(Verse {
            content: "And thou shalt make it an oyle of holy oyntment, an oyntment compound after the arte of the Apothecarie: it shalbe an holy anointing oyle.",
        }),

        ("exodus", 30, 26) => Some(Verse {
            content: "And thou shalt anoint the Tabernacle of the Congregation therewith, and the Arke of the Testimonie:",
        }),

        ("exodus", 30, 27) => Some(Verse {
            content: "And the Table and all his vessels, and the Candlesticke, and his vessels, and the Altar of incense:",
        }),

        ("exodus", 30, 28) => Some(Verse {
            content: "And the Altar of burnt offering with all his vessels, and the Lauer and his foot.",
        }),

        ("exodus", 30, 29) => Some(Verse {
            content: "And thou shalt sanctifie them, that they may bee most holy: whatsoeuer toucheth them, shall be holy.",
        }),

        ("exodus", 30, 30) => Some(Verse {
            content: "And thou shalt annoint Aaron and his sonnes, and consecrate them, that they may minister vnto mee in the priests office.",
        }),

        ("exodus", 30, 31) => Some(Verse {
            content: "And thou shalt speake vnto the children of Israel, saying, This shall bee an holy anointing oile vnto mee, throughout your generations.",
        }),

        ("exodus", 30, 32) => Some(Verse {
            content: "Upon mans flesh shall it not bee powred, neither shall ye make any other like it, after the composition of it: it is holy, and it shall be holy vnto you.",
        }),

        ("exodus", 30, 33) => Some(Verse {
            content: "Whosoeuer compoundeth any like it, or whosoeuer putteth any of it vpon a stranger, shall euen be cut off from his people.",
        }),

        ("exodus", 30, 34) => Some(Verse {
            content: "And the Lord said vnto Moses, Take vnto thee sweete spices, Stacte, and Onicha, and Galbanum: these sweete spices with pure frankincense, of each shall there be a like weight.",
        }),

        ("exodus", 30, 35) => Some(Verse {
            content: "And thou shalt make it a perfume, a confection after the arte of the Apothecarie, tempered together, pure and holy.",
        }),

        ("exodus", 30, 36) => Some(Verse {
            content: "And thou shalt beat some of it very small, and put of it before the testimony in the tabernacle of the Congregation, where I will meet with thee: it shalbe vnto you most holy.",
        }),

        ("exodus", 30, 37) => Some(Verse {
            content: "And as for the perfume which thou shalt make, you shall not make to your selues, according to the composition thereof: it shall be vnto thee holy for the Lord.",
        }),

        ("exodus", 30, 38) => Some(Verse {
            content: "Whosoeuer shall make like vnto that, to smell thereto, shall euen bee cut off from his people.",
        }),

        ("exodus", 31, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 31, 2) => Some(Verse {
            content: "See, I haue called by name, Bezaleel the sonne of Uri, the sonne of Hur, of the tribe of Iudah:",
        }),

        ("exodus", 31, 3) => Some(Verse {
            content: "And I haue filled him with the Spirit of God, in wisedome, and in vnderstanding, and in knowledge, and in all maner of workemanship,",
        }),

        ("exodus", 31, 4) => Some(Verse {
            content: "To deuise cunning workes, to worke in golde, and in siluer, and in brasse,",
        }),

        ("exodus", 31, 5) => Some(Verse {
            content: "And in cutting of stones, to set them, and in caruing of timber, to worke in all maner of workemanship.",
        }),

        ("exodus", 31, 6) => Some(Verse {
            content: "And I, behold, I haue giuen with him, Aholiab the sonne of Ahisamach, of the tribe of Dan, and in the hearts of all that are wise hearted I haue put wisedome, that they may make all that I haue commanded thee:",
        }),

        ("exodus", 31, 7) => Some(Verse {
            content: "The Tabernacle of the Congregation, and the Arke of the Testimony, and the Mercie-seat that is thereupon, & all the furniture of the Tabernacle:",
        }),

        ("exodus", 31, 8) => Some(Verse {
            content: "And the Table, and his furniture, and the pure Candlesticke, with all his furniture, and the Altar of incense:",
        }),

        ("exodus", 31, 9) => Some(Verse {
            content: "And the Altar of burnt offering, with all his furniture, and the Lauer and his foote:",
        }),

        ("exodus", 31, 10) => Some(Verse {
            content: "And the clothes of seruice, and the holy garments for Aaron the Priest, and the garments of his sonnes, to minister in the Priests office:",
        }),

        ("exodus", 31, 11) => Some(Verse {
            content: "And the anointing oyle, and sweet incense for the Holy place: according to all that I haue commanded thee, shall they doe.",
        }),

        ("exodus", 31, 12) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 31, 13) => Some(Verse {
            content: "Speake thou also vnto the children of Israel, saying, Uerely my Sabbaths ye shall keepe: for it is a signe betweene me and you, throughout your generations, that ye may know that I am the Lord, that doth sanctifie you.",
        }),

        ("exodus", 31, 14) => Some(Verse {
            content: "Yee shall keepe the Sabbath therefore: for it is holy vnto you: Euery one that defileth it, shall surely be put to death: for whosoeuer doth any worke therein, that soule shall be cut off from amongst his people.",
        }),

        ("exodus", 31, 15) => Some(Verse {
            content: "Sixe dayes may worke bee done, but in the seuenth is the Sabbath of rest, holy to the Lord: whosoeuer doth any worke in the Sabbath day, he shall surely be put to death.",
        }),

        ("exodus", 31, 16) => Some(Verse {
            content: "Wherefore the children of Israel shall keepe the Sabbath, to obserue the Sabbath throughout their generations, for a perpetuall couenant.",
        }),

        ("exodus", 31, 17) => Some(Verse {
            content: "It is a signe betweene me and the children of Israel for euer: for in sixe dayes the Lord made heauen and earth, and on the seuenth day he rested, and was refreshed.",
        }),

        ("exodus", 31, 18) => Some(Verse {
            content: "And he gaue vnto Moses, when hee had made an end of communing with him vpon mount Sinai, two tables of Testimonie, tables of stone, written with the finger of God.",
        }),

        ("exodus", 32, 1) => Some(Verse {
            content: "And when the people saw that Moses delayed to come downe out of the mount, the people gathered themselues together vnto Aaron, and said vnto him, Up, make vs gods which shall goe before vs: for as for this Moses, the man that brought vs vp out of the land of Egypt, we wot not what is become of him.",
        }),

        ("exodus", 32, 2) => Some(Verse {
            content: "And Aaron saide vnto them, Breake off the golden earerings which are in the eares of your wiues, of your sonnes, and of your daughters, and bring them vnto me.",
        }),

        ("exodus", 32, 3) => Some(Verse {
            content: "And all the people brake off the golden earerings, which were in their eares, and brought them vnto Aaron.",
        }),

        ("exodus", 32, 4) => Some(Verse {
            content: "And hee receiued them at their hand, and fashioned it with a grauing toole, after hee had made it a molten calfe: and they said, These be thy gods, O Israel, which brought thee vp out of the land of Egypt.",
        }),

        ("exodus", 32, 5) => Some(Verse {
            content: "And when Aaron saw it, he built an altar before it, and Aaron made proclamation, and said, To morrow is a feast to the Lord.",
        }),

        ("exodus", 32, 6) => Some(Verse {
            content: "And they rose vp early on the morrow, and offered burnt offerings, and brought peace offerings: and the people sate downe to eate and to drinke, and rose vp to play.",
        }),

        ("exodus", 32, 7) => Some(Verse {
            content: "And the Lord said vnto Moses, Goe, get thee downe: for thy people which thou broughtest out of the land of Egypt, haue corrupted themselues.",
        }),

        ("exodus", 32, 8) => Some(Verse {
            content: "They haue turned aside quickly out of the way which I commaunded them: they haue made them a molten Calfe, and haue worshipped it, and haue sacrificed thereunto, and saide, These bee thy gods, O Israel, which haue brought thee vp out of the land of Egypt.",
        }),

        ("exodus", 32, 9) => Some(Verse {
            content: "And the Lord said vnto Moses, I haue seene this people, and behold, it is a stiffenecked people.",
        }),

        ("exodus", 32, 10) => Some(Verse {
            content: "Now therefore let me alone, that my wrath may waxe hot against them, and that I may consume them: and I will make of thee a great nation.",
        }),

        ("exodus", 32, 11) => Some(Verse {
            content: "And Moses besought the Lord his God, and said, Lord, why doeth thy wrath ware hot against thy people, which thou hast brought foorth out of the land of Egypt, with great power, and with a mighty hand?",
        }),

        ("exodus", 32, 12) => Some(Verse {
            content: "Wherefore should the Egyptians speake and say, For mischiefe did he bring them out, to slay them in the mountaines, & to consume them from the face of the earth? Turne from thy fierce wrath, and repent of this euill against thy people.",
        }),

        ("exodus", 32, 13) => Some(Verse {
            content: "Remember Abraham, Isaac, and Israel thy seruants, to whom thou swarest by thine owne selfe, and saidest vnto them, I will multiply your seed as the starres of heauen: and all this land that I haue spoken of, will I giue vnto your seed, and they shall inherit it for euer.",
        }),

        ("exodus", 32, 14) => Some(Verse {
            content: "And the Lord repented of the euill which he thought to doe vnto his people.",
        }),

        ("exodus", 32, 15) => Some(Verse {
            content: "And Moses turned, and went downe from the Mount, and the two Tables of the Testimony were in his hand: the Tables were written on both their sides; on the one side, and on the other were they written.",
        }),

        ("exodus", 32, 16) => Some(Verse {
            content: "And the Tables were the worke of God; and the writing was the writing of God, grauen vpon the Tables.",
        }),

        ("exodus", 32, 17) => Some(Verse {
            content: "And when Ioshua heard the noise of the people as they shouted, hee said vnto Moses, There is a noise of warre in the campe.",
        }),

        ("exodus", 32, 18) => Some(Verse {
            content: "And he said, It is not the voyce of them that shout for mastery, neither is it the voyce of them that cry for being ouercome: but the noyse of them that sing doe I heare.",
        }),

        ("exodus", 32, 19) => Some(Verse {
            content: "And it came to passe, assoone as he came nigh vnto the campe, that he saw the Calfe, and the dancing: and Moses anger waxed hot, and he cast the Tables out of his hands, and brake them beneath the mount.",
        }),

        ("exodus", 32, 20) => Some(Verse {
            content: "And he tooke the Calfe which they had made, and burnt it in the fire, and ground it to powder, and strawed it vpon the water, and made the children of Israel drinke of it.",
        }),

        ("exodus", 32, 21) => Some(Verse {
            content: "And Moses said vnto Aaron, What did this people vnto thee, that thou hast brought so great a sinne vpon them?",
        }),

        ("exodus", 32, 22) => Some(Verse {
            content: "And Aaron said, Let not the anger of my lord waxe hot: thou knowest the people, that they are set on mischiefe.",
        }),

        ("exodus", 32, 23) => Some(Verse {
            content: "For they said vnto me, Make vs gods which shall goe before vs: for as for this Moses, the man that brought vs vp out of the land of Egypt, we wot not what is become of him.",
        }),

        ("exodus", 32, 24) => Some(Verse {
            content: "And I said vnto them, Whosoeuer hath any gold, let them breake it off: So they gaue it mee: then I cast it into the fire, & there came out this Calfe.",
        }),

        ("exodus", 32, 25) => Some(Verse {
            content: "And when Moses saw that the people were naked, (for Aaron had made them naked vnto their shame, amongst their enemies)",
        }),

        ("exodus", 32, 26) => Some(Verse {
            content: "Then Moses stood in the gate of the campe, and saide, Who is on the Lords side? let him come vnto mee. And all the sonnes of Leui gathered themselues together vnto him.",
        }),

        ("exodus", 32, 27) => Some(Verse {
            content: "And hee said vnto them, Thus saith the Lord God of Israel, Put euery man his sword by his side, and go in and out from gate to gate throughout the campe, and slay euery man his brother, and euery man his companion, and euery man his neighbour.",
        }),

        ("exodus", 32, 28) => Some(Verse {
            content: "And the children of Leui did according to the word of Moses; and there fell of the people that day about three thousand men.",
        }),

        ("exodus", 32, 29) => Some(Verse {
            content: "For Moses had said, Consecrate your selues to day to the Lord, euen euery man vpon his sonne, and vpon his brother, that he may bestow vpon you a blessing this day.",
        }),

        ("exodus", 32, 30) => Some(Verse {
            content: "And it came to passe on the morrow, that Moses said vnto the people, Ye haue sinned a great sinne: And now I will goe vp vnto the Lord; peraduenture I shall make an atonement for your sinne.",
        }),

        ("exodus", 32, 31) => Some(Verse {
            content: "And Moses returned vnto the Lord, and said, Oh, this people haue sinned a great sinne, and haue made them gods of gold.",
        }),

        ("exodus", 32, 32) => Some(Verse {
            content: "Yet now, if thou wilt forgiue their sinne; and if not, blot me, I pray thee, out of thy Booke, which thou hast written.",
        }),

        ("exodus", 32, 33) => Some(Verse {
            content: "And the Lord said vnto Moses, Whosoeuer hath sinned against me, him will I blot out of my Booke.",
        }),

        ("exodus", 32, 34) => Some(Verse {
            content: "Therefore now goe, leade the people vnto the place of which I haue spoken vnto thee: Behold, mine Angel shall goe before thee; Neuerthelesse in the day when I visit, I will visit their sinne vpon them.",
        }),

        ("exodus", 32, 35) => Some(Verse {
            content: "And the Lord plagued the people, because they made the Calfe, which Aaron made.",
        }),

        ("exodus", 33, 1) => Some(Verse {
            content: "And the Lord said vnto Moses, Depart, and goe vp hence, thou and the people which thou hast brought vp out of the land of Egypt, vnto the land which I sware vnto Abraham, to Isaac, & to Iacob, saying, Unto thy seed will I giue it.",
        }),

        ("exodus", 33, 2) => Some(Verse {
            content: "And I will send an Angel before thee, and I will driue out the Canaanite, the Amorite, and the Hittite, and the Perizzite, the Hiuite, and the Iebusite:",
        }),

        ("exodus", 33, 3) => Some(Verse {
            content: "Unto a land flowing with milke and hony: For I will not goe vp in the midst of thee: for thou art a stiffenecked people, lest I consume thee in the way.",
        }),

        ("exodus", 33, 4) => Some(Verse {
            content: "And when the people heard these euill tidings, they mourned: and no man did put on him his ornaments.",
        }),

        ("exodus", 33, 5) => Some(Verse {
            content: "For the Lord had saide vnto Moses, Say vnto the children of Israel, Ye are a stiffenecked people: I wil come vp into the midst of thee in a moment, & consume thee: Therefore now put off thy ornaments from thee, that I may know what to doe vnto thee.",
        }),

        ("exodus", 33, 6) => Some(Verse {
            content: "And the children of Israel stript themselues of their ornaments, by the mount Horeb.",
        }),

        ("exodus", 33, 7) => Some(Verse {
            content: "And Moses tooke the Tabernacle, & pitched it without the campe, a farre off from the campe, and called it the Tabernacle of the Congregation: And it came to passe, that euery one which sought the Lord, went out vnto the Tabernacle of the Congregation, which was without the campe.",
        }),

        ("exodus", 33, 8) => Some(Verse {
            content: "And it came to passe when Moses went out vnto the Tabernacle, that all the people rose vp, and stood euery man at his tent doore, and looked after Moses, vntill he was gone into the Tabernacle.",
        }),

        ("exodus", 33, 9) => Some(Verse {
            content: "And it came to passe as Moses entred into the Tabernacle, the cloudy pillar descended, and stood at the doore of the Tabernacle, and the Lord talked with Moses.",
        }),

        ("exodus", 33, 10) => Some(Verse {
            content: "And all the people saw the cloudy pillar stand at the Tabernacle doore: and all the people rose vp, and worshipped euery man in his tent doore.",
        }),

        ("exodus", 33, 11) => Some(Verse {
            content: "And the Lord spake vnto Moses face to face, as a man speaketh vnto his friend. And he turned againe into the campe, but his seruant Ioshua the sonne of Nun, a yong man, departed not out of the Tabernacle.",
        }),

        ("exodus", 33, 12) => Some(Verse {
            content: "And Moses saide vnto the Lord, See, thou sayest vnto mee, Bring vp this people, and thou hast not let mee know whome thou wilt send with me. Yet thou hast said, I knowe thee by name, and thou hast also found grace in my sight.",
        }),

        ("exodus", 33, 13) => Some(Verse {
            content: "Now therefore, I pray thee, If I haue found grace in thy sight, shewe mee now thy way that I may know thee, that I may find grace in thy sight: and consider that this nation is thy people.",
        }),

        ("exodus", 33, 14) => Some(Verse {
            content: "And he said, My presence shall go with thee, and I will giue thee rest.",
        }),

        ("exodus", 33, 15) => Some(Verse {
            content: "And he said vnto him, If thy presence goe not with mee, carie vs not vp hence.",
        }),

        ("exodus", 33, 16) => Some(Verse {
            content: "For wherein shall it bee knowen here, that I and thy people haue found grace in thy sight? is it not in that thou goest with vs? So shall we be separated, I and thy people, from all the people that are vpon the face of the earth.",
        }),

        ("exodus", 33, 17) => Some(Verse {
            content: "And the Lord said vnto Moses, I will doe this thing also that thou hast spoken: for thou hast found grace in my sight, and I know thee by name.",
        }),

        ("exodus", 33, 18) => Some(Verse {
            content: "And he said, I beseech thee, shew me thy glory.",
        }),

        ("exodus", 33, 19) => Some(Verse {
            content: "And he said, I will make all my goodnesse passe before thee, and I will proclaime the name of the Lord before thee: and will bee gracious to whom I wil be gracious, and wil shew mercie on whom I will shew mercie.",
        }),

        ("exodus", 33, 20) => Some(Verse {
            content: "And he said, Thou canst not see my face: for there shall no man see mee, and liue.",
        }),

        ("exodus", 33, 21) => Some(Verse {
            content: "And the Lord said, Beholde, there is a place by mee, and thou shalt stand vpon a rocke.",
        }),

        ("exodus", 33, 22) => Some(Verse {
            content: "And it shall come to passe, while my glory passeth by, that I will put thee in a clift of the rocke, and will couer thee with my hand, while I passe by.",
        }),

        ("exodus", 33, 23) => Some(Verse {
            content: "And I wil take away mine hand, and thou shalt see my backe parts: but my face shall not be seene.",
        }),

        ("exodus", 34, 1) => Some(Verse {
            content: "And the Lord said vnto Moses, Hew thee two Tables of stone, like vnto the first: and I will write vpon these Tables, the words that were in the first Tables which thou brakest.",
        }),

        ("exodus", 34, 2) => Some(Verse {
            content: "And be ready in the morning, and come vp in the morning vnto mount Sinai, and present thy selfe there to me, in the top of the mount.",
        }),

        ("exodus", 34, 3) => Some(Verse {
            content: "And no man shall come vp with thee, neither let any man bee seene throughout all the mount, neither let the flockes nor herds feede before that mount.",
        }),

        ("exodus", 34, 4) => Some(Verse {
            content: "And he hewed two Tables of stone, like vnto the first, and Moses rose vp earely in the morning, and went vp vnto mount Sinai, as the Lord had commanded him, and tooke in his hand the two tables of stone.",
        }),

        ("exodus", 34, 5) => Some(Verse {
            content: "And the Lord descended in the cloud, and stood with him there, and proclaimed the Name of the Lord.",
        }),

        ("exodus", 34, 6) => Some(Verse {
            content: "And the Lord passed by before him, and proclaimed, The Lord, The Lord God, mercifull and gracious, long suffering, and abundant in goodnesse and trueth,",
        }),

        ("exodus", 34, 7) => Some(Verse {
            content: "Keeping mercie for thousands, forgiuing iniquitie and transgression and sinne, and that will by no meanes cleere the guiltie, visiting the iniquitie of the fathers vpon the children, and vpon the childrens children, vnto the third and to the fourth generation.",
        }),

        ("exodus", 34, 8) => Some(Verse {
            content: "And Moses made haste, and bowed his head toward the earth, and worshipped.",
        }),

        ("exodus", 34, 9) => Some(Verse {
            content: "And he said, If now I haue found grace in thy sight, O Lord, let my Lord, I pray thee, goe amongst vs, (for it is a stiffenecked people,) and pardon our iniquitie, and our sinne, and take vs for thine inheritance.",
        }),

        ("exodus", 34, 10) => Some(Verse {
            content: "And he said, Behold, I make a couenant: before all thy people, I wil doe marueiles, such as haue not beene done in all the earth, nor in any nation: and all the people amongst which thou art, shall see the worke of the Lord: for it is a terrible thing that I will doe with thee.",
        }),

        ("exodus", 34, 11) => Some(Verse {
            content: "Obserue thou that which I command thee this day: Behold, I driue out before thee the Amorite, and the Canaanite, and the Hittite, and the Perizzite, and the Hiuite, and the Iebusite.",
        }),

        ("exodus", 34, 12) => Some(Verse {
            content: "Take heed to thy selfe, lest thou make a couenant with the inhabitants of the land whither thou goest, lest it be for a snare in the midst of thee.",
        }),

        ("exodus", 34, 13) => Some(Verse {
            content: "But ye shall destroy their altars, breake their images, and cut downe their groues.",
        }),

        ("exodus", 34, 14) => Some(Verse {
            content: "For thou shalt worship no other god: for the Lord, whose name is Ielous, is a Ielous God:",
        }),

        ("exodus", 34, 15) => Some(Verse {
            content: "Lest thou make a couenant with the inhabitants of the land, and they goe a whoring after their gods, and doe sacrifice vnto their gods, and one call thee, and thou eate of his sacrifice,",
        }),

        ("exodus", 34, 16) => Some(Verse {
            content: "And thou take of their daughters vnto thy sonnes, and their daughters goe a whoring after their gods, and make thy sonnes goe a whoring after their gods.",
        }),

        ("exodus", 34, 17) => Some(Verse {
            content: "Thou shalt make thee no molten gods.",
        }),

        ("exodus", 34, 18) => Some(Verse {
            content: "The feast of vnleauened bread shalt thou keepe: Seuen dayes thou shalt eate vnleauened bread, as I commanded thee in the time of the moneth Abib: for in the moneth Abib thou camest out from Egypt.",
        }),

        ("exodus", 34, 19) => Some(Verse {
            content: "All that openeth the matrixe is mine: and euery firstling amongst thy cattell, whether oxe or sheepe, that is male.",
        }),

        ("exodus", 34, 20) => Some(Verse {
            content: "But the firstling of an Asse thou shalt redeeme with a lambe: and if thou redeeme him not, then shalt thou breake his necke. All the first borne of thy sonnes thou shalt redeeme: and none shall appeare before me empty.",
        }),

        ("exodus", 34, 21) => Some(Verse {
            content: "Sixe dayes thou shalt worke, but on the seuenth day thou shalt rest: in earing time and in haruest thou shalt rest.",
        }),

        ("exodus", 34, 22) => Some(Verse {
            content: "And thou shalt obserue the feast of weekes, of the first fruits of wheat haruest, and the feast of ingathering at the yeeres end.",
        }),

        ("exodus", 34, 23) => Some(Verse {
            content: "Thrice in the yeere shall all your men children appeare before the Lord God, the God of Israel.",
        }),

        ("exodus", 34, 24) => Some(Verse {
            content: "For I will cast out the nations before thee, and enlarge thy borders: neither shall any man desire thy land, when thou shalt goe vp to appeare before the Lord thy God, thrice in the yeere.",
        }),

        ("exodus", 34, 25) => Some(Verse {
            content: "Thou shalt not offer the blood of my sacrifice with leauen, neither shall the sacrifice of the feast of Passeouer be left vnto the morning.",
        }),

        ("exodus", 34, 26) => Some(Verse {
            content: "The first of the first fruits of thy land thou shalt bring vnto the house of the Lord thy God. Thou shalt not seeth a kid in his mothers milke.",
        }),

        ("exodus", 34, 27) => Some(Verse {
            content: "And the Lord said vnto Moses, Write thou these words: for after the tenour of these wordes, I haue made a couenant with thee, and with Israel.",
        }),

        ("exodus", 34, 28) => Some(Verse {
            content: "And hee was there with the Lord forty dayes and forty nights: he did neither eat bread, nor drinke water; and he wrote vpon the Tables the words of the couenant, the ten Commandements.",
        }),

        ("exodus", 34, 29) => Some(Verse {
            content: "And it came to passe when Moses came downe from mount Sinai (with the two Tables of Testimony in Moses hand, when hee came downe from the mount) that Moses wist not that the skin of his face shone, while he talked with him.",
        }),

        ("exodus", 34, 30) => Some(Verse {
            content: "And when Aaron and all the children of Israel saw Moses, behold, the skinne of his face shone, and they were afraid to come nigh him.",
        }),

        ("exodus", 34, 31) => Some(Verse {
            content: "And Moses called vnto them, and Aaron and all the rulers of the Congregation returned vnto him, and Moses talked with them.",
        }),

        ("exodus", 34, 32) => Some(Verse {
            content: "And afterward all the children of Israel came nigh: and he gaue them in commandement all that the Lord had spoken with him in mount Sinai.",
        }),

        ("exodus", 34, 33) => Some(Verse {
            content: "And till Moses had done speaking with them, he put a vaile on his face.",
        }),

        ("exodus", 34, 34) => Some(Verse {
            content: "But when Moses went in before the Lord to speake with him, hee tooke the vaile off, vntill he came out: And hee came out and spake vnto the children of Israel, that which he was commanded.",
        }),

        ("exodus", 34, 35) => Some(Verse {
            content: "And the children of Israel saw the face of Moses, that the skinne of Moses face shone: and Moses put the vaile vpon his face againe, vntill hee went in to speake with him.",
        }),

        ("exodus", 35, 1) => Some(Verse {
            content: "And Moses gathered all the Congregation of the children of Israel together, and said vnto them; These are the wordes which the Lord hath commanded, that yee should doe them.",
        }),

        ("exodus", 35, 2) => Some(Verse {
            content: "Sixe dayes shall worke be done, but on the seuenth day there shall be to you an holy day, a Sabbath of rest to the Lord: whosoeuer doeth worke therein, shall be put to death.",
        }),

        ("exodus", 35, 3) => Some(Verse {
            content: "Ye shall kindle no fire throughout your habitations vpon the Sabbath day.",
        }),

        ("exodus", 35, 4) => Some(Verse {
            content: "And Moses spake vnto all the Congregation of the children of Israel, saying, This is the thing which the Lord commanded, saying,",
        }),

        ("exodus", 35, 5) => Some(Verse {
            content: "Take ye from amongst you an offring vnto the Lord: Whosoeuer is of a willing heart, let him bring it, an offering of the Lord, gold, and siluer, and brasse,",
        }),

        ("exodus", 35, 6) => Some(Verse {
            content: "And blew, and purple, and scarlet, and fine linnen, and goats haire,",
        }),

        ("exodus", 35, 7) => Some(Verse {
            content: "And rammes skinnes died red, & badgers skinnes, and Shittim wood,",
        }),

        ("exodus", 35, 8) => Some(Verse {
            content: "And oyle for the light, and spices for anoynting oyle, and for the sweet incense:",
        }),

        ("exodus", 35, 9) => Some(Verse {
            content: "And Onix stones, and stones to be set for the Ephod, and for the brestplate.",
        }),

        ("exodus", 35, 10) => Some(Verse {
            content: "And euery wise hearted among you, shall come and make all that the Lord hath commanded:",
        }),

        ("exodus", 35, 11) => Some(Verse {
            content: "The Tabernacle, his tent, and his couering, his taches, & his barres, his pillars, and his sockets:",
        }),

        ("exodus", 35, 12) => Some(Verse {
            content: "The Arke and the staues thereof, with the Mercy seat, and the Uaile of the couering:",
        }),

        ("exodus", 35, 13) => Some(Verse {
            content: "The Table and his staues, and all his vessels, and the Shewbread,",
        }),

        ("exodus", 35, 14) => Some(Verse {
            content: "The Candlesticke also for the light, and his furniture, and his lamps, with the oyle for the light,",
        }),

        ("exodus", 35, 15) => Some(Verse {
            content: "And the incense Altar, and his staues, and the anoynting oyle, and the sweet incense, and the hanging for the doore, at the entring in of the Tabernacle:",
        }),

        ("exodus", 35, 16) => Some(Verse {
            content: "The Altar of burnt offering with his brasen grate, his staues, and all his vessels, the Lauer and his foot:",
        }),

        ("exodus", 35, 17) => Some(Verse {
            content: "The hangings of the Court, his pillars, and their sockets, and the hanging for the doore of the Court:",
        }),

        ("exodus", 35, 18) => Some(Verse {
            content: "The pinnes of the Tabernacle, and the pinnes of the Court, and their coards:",
        }),

        ("exodus", 35, 19) => Some(Verse {
            content: "The cloathes of seruice, to doe seruice in the holy place, the holy garments for Aaron the Priest, and the garments of his sonnes to minister in the Priests office.",
        }),

        ("exodus", 35, 20) => Some(Verse {
            content: "And all the Congregation of the children of Israel departed from the presence of Moses.",
        }),

        ("exodus", 35, 21) => Some(Verse {
            content: "And they came euery one whose heart stirred him vp, and euery one whom his spirit made willing, and they brought the Lords offering to the worke of the Tabernacle of the Congregation, and for all his seruice, and for the holy garments.",
        }),

        ("exodus", 35, 22) => Some(Verse {
            content: "And they came both men and women, as many as were willing hearted, and brought bracelets, and earerings, and rings, & tablets, all iewels of gold: and euery man that offered, offered an offering of gold vnto the Lord.",
        }),

        ("exodus", 35, 23) => Some(Verse {
            content: "And euery man with whom was found blew, and purple, and scarlet, and fine linnen, and goates haire, and red skinnes of rammes, and badgers skinnes, brought them.",
        }),

        ("exodus", 35, 24) => Some(Verse {
            content: "Euery one that did offer an offering of siluer and brasse, brought the Lords offering: and euery man with whom was found Shittim wood for any worke of the seruice, brought it.",
        }),

        ("exodus", 35, 25) => Some(Verse {
            content: "And all the women that were wise hearted, did spin with their hands, and brought that which they had spun, both of blew, and of purple, and of scarlet, and of fine linnen.",
        }),

        ("exodus", 35, 26) => Some(Verse {
            content: "And all the women whose heart stirred them vp in wisedome, spunne goats haire.",
        }),

        ("exodus", 35, 27) => Some(Verse {
            content: "And the rulers brought Onix stones, and stones to be set for the Ephod, and for the brestplate:",
        }),

        ("exodus", 35, 28) => Some(Verse {
            content: "And spice, and oyle for the light, and for the anoynting oyle, and for the sweet incense.",
        }),

        ("exodus", 35, 29) => Some(Verse {
            content: "The children of Israel brought a willing offering vnto the Lord, euery man and woman, whose heart made them willing to bring for all maner of worke, which the Lord had commanded to be made by the hands of Moses.",
        }),

        ("exodus", 35, 30) => Some(Verse {
            content: "And Moses said vnto the children of Israel, See, the Lord hath called by name Bezaleel the sonne of Uri, the sonne of Hur, of the tribe of Iudah.",
        }),

        ("exodus", 35, 31) => Some(Verse {
            content: "And he hath filled him with the Spirit of God, in wisedome, in vnderstanding, and in knowledge, and in all maner of workemanship:",
        }),

        ("exodus", 35, 32) => Some(Verse {
            content: "And to deuise curious workes, to worke in gold, & in siluer, and in brasse,",
        }),

        ("exodus", 35, 33) => Some(Verse {
            content: "And in the cutting of stones, to set them, and in caruing of wood, to make any maner of cunning worke.",
        }),

        ("exodus", 35, 34) => Some(Verse {
            content: "And he hath put in his heart that he may teach, both he and Aholiab the sonne of Ahisamach of the tribe of Dan.",
        }),

        ("exodus", 35, 35) => Some(Verse {
            content: "Them hath hee filled with wisedome of heart, to worke all manner of worke, of the ingrauer, and of the cunning workeman, and of the embroiderer, in blew, and in purple, in scarlet, and in fine linnen, and of the weauer, euen of them that doe any worke, and of those that deuise cunning worke.",
        }),

        ("exodus", 36, 1) => Some(Verse {
            content: "Then wrought Bezaleel and Aholiab, and euery wise hearted man, in whome the Lord put wisedome and vnderstanding, to know how to worke all maner of worke for the seruice of the Sanctuary, according to all that the Lord had commanded.",
        }),

        ("exodus", 36, 2) => Some(Verse {
            content: "And Moses called Bezaleel and Aholiab, and euery wise hearted man, in whose heart the Lord had put wisedome, euen euery one whose heart stirred him vp to come vnto the worke to doe it.",
        }),

        ("exodus", 36, 3) => Some(Verse {
            content: "And they receiued of Moses all the offering which the children of Israel had brought, for the worke of the seruice of the Sanctuarie, to make it withall. And they brought yet vnto him free offerings euery morning.",
        }),

        ("exodus", 36, 4) => Some(Verse {
            content: "And al the wisemen that wrought all the worke of the Sanctuary, came euery man from his worke which they made.",
        }),

        ("exodus", 36, 5) => Some(Verse {
            content: "And they spake vnto Moses, saying, The people bring much more then enough for the seruice of the worke which the Lord commaunded to make.",
        }),

        ("exodus", 36, 6) => Some(Verse {
            content: "And Moses gaue commandement, and they caused it to bee proclaimed throughout the campe, saying, Let neither man nor woman make any more worke for the offering of the Sanctuarie: so the people were restrained from bringing.",
        }),

        ("exodus", 36, 7) => Some(Verse {
            content: "For the stuffe they had was sufficient for all the worke to make it, and too much.",
        }),

        ("exodus", 36, 8) => Some(Verse {
            content: "And euery wise hearted man, among them that wrought the worke of the Tabernacle, made ten curtaines, of fine twined linnen, and blew, and purple, and scarlet: with Cherubims of cunning worke made he them.",
        }),

        ("exodus", 36, 9) => Some(Verse {
            content: "The length of one curtaine was twentie & eight cubites, and the breadth of one curtaine foure cubites: the curtaines were all of one cise.",
        }),

        ("exodus", 36, 10) => Some(Verse {
            content: "And he coupled the fiue curtaines one vnto another: and the other fiue curtaines he coupled one vnto another.",
        }),

        ("exodus", 36, 11) => Some(Verse {
            content: "And he made loopes of blew, on the edge of one curtaine, from the seluedge in the coupling: likewise hee made in the vttermost side of another curtaine, in the coupling of the second.",
        }),

        ("exodus", 36, 12) => Some(Verse {
            content: "Fiftie loopes made he in one curtaine, and fiftie loopes made hee in the edge of the curtaine which was in the coupling of the second: the loopes held one curtaine to another.",
        }),

        ("exodus", 36, 13) => Some(Verse {
            content: "And he made fiftie taches of gold, and coupled the curtaines one vnto another with the taches. So it became one tabernacle.",
        }),

        ("exodus", 36, 14) => Some(Verse {
            content: "And he made curtaines of goats haire, for the tent ouer the Tabernacle: eleuen curtaines he made them.",
        }),

        ("exodus", 36, 15) => Some(Verse {
            content: "The length of one curtaine was thirtie cubites, and foure cubites was the breadth of one curtaine: the eleuen curtaines were of one cise.",
        }),

        ("exodus", 36, 16) => Some(Verse {
            content: "And he coupled fiue curtaines by themselues, and sixe curtaines by themselues.",
        }),

        ("exodus", 36, 17) => Some(Verse {
            content: "And he made fiftie loopes vpon the vttermost edge of the curtaine in the coupling, and fiftie loopes made he vpon the edge of the curtaine, which coupleth the second.",
        }),

        ("exodus", 36, 18) => Some(Verse {
            content: "And he made fiftie taches of brasse to couple the tent together that it might be one.",
        }),

        ("exodus", 36, 19) => Some(Verse {
            content: "And he made a couering for the tent of rammes skinnes died red, and a couering of badgers skinnes aboue that.",
        }),

        ("exodus", 36, 20) => Some(Verse {
            content: "And hee made boards for the Tabernacle of Shittim wood, standing up.",
        }),

        ("exodus", 36, 21) => Some(Verse {
            content: "The length of a board was ten cubites, and the breadth of a board one cubite and a halfe.",
        }),

        ("exodus", 36, 22) => Some(Verse {
            content: "One board had two tenons, equally distant one from another: thus did he make for all the boards of the tabernacle.",
        }),

        ("exodus", 36, 23) => Some(Verse {
            content: "And he made boards for the Tabernacle: twentie boards for the South side, Southward.",
        }),

        ("exodus", 36, 24) => Some(Verse {
            content: "And fourtie sockets of siluer hee made vnder the twentie boards: two sockets vnder one board for his two tenons, and two sockets vnder another board, for his two tenons.",
        }),

        ("exodus", 36, 25) => Some(Verse {
            content: "And for the other side of the Tabernacle which is toward the North corner, he made twentie boards.",
        }),

        ("exodus", 36, 26) => Some(Verse {
            content: "And their fourtie sockets of siluer: two sockets vnder one board, and two sockets vnder another board.",
        }),

        ("exodus", 36, 27) => Some(Verse {
            content: "And for the sides of the Tabernacle westward, he made sixe boards.",
        }),

        ("exodus", 36, 28) => Some(Verse {
            content: "And two boards made he for the corners of the Tabernacle, in the two sides.",
        }),

        ("exodus", 36, 29) => Some(Verse {
            content: "And they were coupled beneath and coupled together at the head thereof, to one ring: thus hee did to both of them in both the corners.",
        }),

        ("exodus", 36, 30) => Some(Verse {
            content: "And there were eight boards, and their sockets were sixteene sockets of siluer: vnder euery board two sockets.",
        }),

        ("exodus", 36, 31) => Some(Verse {
            content: "And he made barres of Shittim wood: five for the boards of the one side of the Tabernacle,",
        }),

        ("exodus", 36, 32) => Some(Verse {
            content: "And fiue barres for the boards of the other side of the Tabernacle, and fiue barres for the boards of the Tabernacle for the sides westward.",
        }),

        ("exodus", 36, 33) => Some(Verse {
            content: "And he made the middle barre to shoot thorow the boards from the one end to the other.",
        }),

        ("exodus", 36, 34) => Some(Verse {
            content: "And he ouerlaid the boards with gold, and made their rings of golde to be places for the barres, and ouerlaide the barres with gold.",
        }),

        ("exodus", 36, 35) => Some(Verse {
            content: "And he made a Uaile of blew, and purple, and scarlet, and fine twined linnen: with Cherubims made he it of cunning worke.",
        }),

        ("exodus", 36, 36) => Some(Verse {
            content: "And he made thereunto foure pillars of Shittim wood, and ouerlaide them with golde: their hookes were of gold: and he cast for them foure sockets of siluer.",
        }),

        ("exodus", 36, 37) => Some(Verse {
            content: "And hee made an hanging for the Tabernacle doore of blew and purple, and scarlet, and fine twined linnen, of needle worke,",
        }),

        ("exodus", 36, 38) => Some(Verse {
            content: "And the fiue pillars of it with their hooks: and he ouerlaid their chapiters and their fillets with gold: but their fiue sockets were of brasse.",
        }),

        ("exodus", 37, 1) => Some(Verse {
            content: "And Bezaleel made the Arke of Shittim wood: two cubites and a halfe was the length of it, and a cubite and a halfe the breadth of it, and a cubite and a halfe the height of it.",
        }),

        ("exodus", 37, 2) => Some(Verse {
            content: "And he ouerlaid it with pure gold within & without, and made a crowne of gold to it round about.",
        }),

        ("exodus", 37, 3) => Some(Verse {
            content: "And hee cast for it foure rings of gold, to be set by the foure corners of it: euen two rings vpon the one side of it, and two rings vpon the other side of it.",
        }),

        ("exodus", 37, 4) => Some(Verse {
            content: "And he made staues of Shittim wood, and ouerlaid them with gold.",
        }),

        ("exodus", 37, 5) => Some(Verse {
            content: "And hee put the staues into the rings, by the sides of the Arke, to beare the Arke.",
        }),

        ("exodus", 37, 6) => Some(Verse {
            content: "And he made the Mercie seat of pure gold: two cubites and an halfe was the length thereof, and one cubite and an halfe the breadth thereof.",
        }),

        ("exodus", 37, 7) => Some(Verse {
            content: "And he made two Cherubims of gold, beaten out of one piece made hee them, on the two endes of the Mercie seate:",
        }),

        ("exodus", 37, 8) => Some(Verse {
            content: "One Cherub on the end on this side, and another Cherub on the other end, on that side: out of the Mercie seat made hee the Cherubims on the two ends thereof.",
        }),

        ("exodus", 37, 9) => Some(Verse {
            content: "And the Cherubims spread out their wings on high, and couered with their wings ouer the Mercie seat with their faces one to another: euen to the Mercie seat ward were the faces of the Cherubims.",
        }),

        ("exodus", 37, 10) => Some(Verse {
            content: "And hee made the Table of Shittim wood: two cubites was the length thereof, and a cubite the breadth thereof, and a cubite and a halfe the height thereof.",
        }),

        ("exodus", 37, 11) => Some(Verse {
            content: "And he ouerlaid it with pure gold, and made thereunto a crowne of gold round about.",
        }),

        ("exodus", 37, 12) => Some(Verse {
            content: "Also he made thereunto a border of an handbreadth, round about: and made a crowne of gold for the border thereof round about.",
        }),

        ("exodus", 37, 13) => Some(Verse {
            content: "And hee cast for it foure rings of gold, and put the rings vpon the foure corners that were in the foure feete thereof.",
        }),

        ("exodus", 37, 14) => Some(Verse {
            content: "Ouer against the border were the rings, the places for the staues, to beare the Table.",
        }),

        ("exodus", 37, 15) => Some(Verse {
            content: "And he made the staues of Shittim wood, and ouerlayed them with gold, to beare the Table.",
        }),

        ("exodus", 37, 16) => Some(Verse {
            content: "And hee made the vessels which were vpon the Table, his dishes, and his spoones, and his bowles, and his couers to couer withall, of pure gold.",
        }),

        ("exodus", 37, 17) => Some(Verse {
            content: "And he made the Candlesticke of pure gold, of beaten worke made he the Candlesticke, his shaft & his branch, his bowles, his knops, and his flowers were of the same.",
        }),

        ("exodus", 37, 18) => Some(Verse {
            content: "And sixe branches going out of the sides thereof: three branches of the candlesticke out of the one side thereof, and three branches of the candlesticke out of the other side thereof.",
        }),

        ("exodus", 37, 19) => Some(Verse {
            content: "Three bowles made he after the fashion of almonds, in one branch, a knop and a flower: and three bowles made like almonds, in another branch, a knop and a flower: so throughout the sixe branches, going out of the Candlesticke.",
        }),

        ("exodus", 37, 20) => Some(Verse {
            content: "And in the candlesticke were foure bowles made like almonds, his knops, and his flowers:",
        }),

        ("exodus", 37, 21) => Some(Verse {
            content: "And a knop vnder two branches of the same, & a knop vnder two branches of the same, and a knop vnder two branches of the same, according to the sixe branches going out of it.",
        }),

        ("exodus", 37, 22) => Some(Verse {
            content: "Their knops and their branches were of the same: all of it was one beaten worke of pure gold.",
        }),

        ("exodus", 37, 23) => Some(Verse {
            content: "And he made his seuen lampes, and his snuffers, and his snuffe-dishes of pure gold.",
        }),

        ("exodus", 37, 24) => Some(Verse {
            content: "Of a talent of pure gold made he it, and all the vessels thereof.",
        }),

        ("exodus", 37, 25) => Some(Verse {
            content: "And he made the incense Altar of Shittim wood: the length of it was a cubit, and the breadth of it a cubit: it was foure square, and two cubites was the height of it; the hornes thereof were of the same.",
        }),

        ("exodus", 37, 26) => Some(Verse {
            content: "And he ouerlayed it with pure gold, both the top of it and the sides thereof round about, and the hornes of it: also he made vnto it a crowne of gold round about.",
        }),

        ("exodus", 37, 27) => Some(Verse {
            content: "And he made two rings of gold for it vnder the crowne thereof, by the two corners of it, vpon the two sides thereof, to bee places for the staues to beare it withall.",
        }),

        ("exodus", 37, 28) => Some(Verse {
            content: "And he made the staues of Shittim wood, and ouerlayed them with gold.",
        }),

        ("exodus", 37, 29) => Some(Verse {
            content: "And he made the holy anoynting oyle, and the pure incense of sweet spices, according to the worke of the Apothecary.",
        }),

        ("exodus", 38, 1) => Some(Verse {
            content: "And he made the Altar of burnt offring of Shittim wood: fiue cubits was the length thereof, and fiue cubits the breadth thereof: it was foure square, and three cubits the height thereof.",
        }),

        ("exodus", 38, 2) => Some(Verse {
            content: "And hee made the hornes thereof on the foure corners of it: the hornes thereof were of the same, and he ouerlayed it with brasse.",
        }),

        ("exodus", 38, 3) => Some(Verse {
            content: "And he made all the vessels of the Altar, the pots and the shouels, and the basons, and the fleshhookes, and the firepannes: all the vessels thereof made he of brasse.",
        }),

        ("exodus", 38, 4) => Some(Verse {
            content: "And he made for the Altar a brasen grate of networke, vnder the compasse thereof, beneath vnto the midst of it.",
        }),

        ("exodus", 38, 5) => Some(Verse {
            content: "And hee cast foure rings for the foure ends of the grate of brasse, to bee places for the staues.",
        }),

        ("exodus", 38, 6) => Some(Verse {
            content: "And he made the staues of Shittim wood, and ouerlayed them with brasse.",
        }),

        ("exodus", 38, 7) => Some(Verse {
            content: "And hee put the staues into the rings on the sides of the Altar, to beare it withall; hee made the Altar hollow with boards.",
        }),

        ("exodus", 38, 8) => Some(Verse {
            content: "And hee made the Lauer of brasse, and the foot of it of brasse, of the looking glasses of the women assembling, which assembled at the doore of the Tabernacle of the Congregation.",
        }),

        ("exodus", 38, 9) => Some(Verse {
            content: "And he made the Court: on the Southside Southward, the hangings of the Court were of fine twined linnen, a hundred cubits.",
        }),

        ("exodus", 38, 10) => Some(Verse {
            content: "Their pillars were twenty, and their brasen sockets twentie: the hooks of the pillars, and their fillets were of siluer.",
        }),

        ("exodus", 38, 11) => Some(Verse {
            content: "And for the North side, the hangings were an hundred cubites, their pillars were twentie, and their sockets of brasse twentie: the hoopes of the pillars, and their fillets of siluer.",
        }),

        ("exodus", 38, 12) => Some(Verse {
            content: "And for the West side were hangings of fiftie cubites, their pillars ten, and their sockets ten: the hookes of the pillars, and their fillets of siluer.",
        }),

        ("exodus", 38, 13) => Some(Verse {
            content: "And for the East side Eastward fiftie cubites.",
        }),

        ("exodus", 38, 14) => Some(Verse {
            content: "The hangings of the one side of the gate were fifteene cubites, their pillars three, and their sockets three.",
        }),

        ("exodus", 38, 15) => Some(Verse {
            content: "And for the other side of the court gate on this hand and that hand were hangings of fifteene cubites, their pillars three, and their sockets three.",
        }),

        ("exodus", 38, 16) => Some(Verse {
            content: "All the hangings of the court round about, were of fine twined linnen.",
        }),

        ("exodus", 38, 17) => Some(Verse {
            content: "And the sockets for the pillars were of brasse, the hookes of the pillars, and their fillets of siluer, and the ouerlaying of their chapiters of siluer, and all the pillars of the court were filleted with siluer.",
        }),

        ("exodus", 38, 18) => Some(Verse {
            content: "And the hanging for the gate of the Court was needle worke, of blew, and purple, and scarlet, and fine twined linnen: and twentie cubites was the length, and the height in the breadth was fiue cubites, answerable to the hangings of the Court.",
        }),

        ("exodus", 38, 19) => Some(Verse {
            content: "And their pillars were foure, and their sockets of brasse foure, their hookes of siluer, and the ouerlaying of their chapiters, & their fillets of siluer.",
        }),

        ("exodus", 38, 20) => Some(Verse {
            content: "And all the pinnes of the Tabernacle, and of the court round about, were of brasse.",
        }),

        ("exodus", 38, 21) => Some(Verse {
            content: "This is the summe of the Tabernacle, euen of the Tabernacle of Testimonie, as it was counted, according to the commaundement of Moses, for the seruice of the Leuites, by the hand of Ithamar, son to Aaron the Priest.",
        }),

        ("exodus", 38, 22) => Some(Verse {
            content: "And Bezaleel the sonne of Uri, the sonne of Hur, of the tribe of Iudah, made all that the Lord commanded Moses.",
        }),

        ("exodus", 38, 23) => Some(Verse {
            content: "And with him was Aholiab, sonne of Ahisamach, of the tribe of Dan, an engrauer, and a cunning workeman, and an embroiderer in blew, and in purple, and in scarlet, and fine linnen.",
        }),

        ("exodus", 38, 24) => Some(Verse {
            content: "All the gold that was occupied for the worke in all the worke of the holy place, euen the gold of the offring, was twentie and nine talents, and seuen hundred and thirtie shekels, after the shekel of the Sanctuary.",
        }),

        ("exodus", 38, 25) => Some(Verse {
            content: "And the siluer of them that were numbred of the Congregation, was an hundred talents, and a thousand, seuen hundred and threescore and fifteene shekels, after the shekel of the Sanctuary.",
        }),

        ("exodus", 38, 26) => Some(Verse {
            content: "A Bekah for euery man, that is, halfe a shekel, after the shekel of the Sanctuary, for euery one that went to be numbred, from twentie yeeres olde and vpward, for sixe hundred thousand, and three thousand, and fiue hundred, and fiftie men.",
        }),

        ("exodus", 38, 27) => Some(Verse {
            content: "And of the hundred talents of siluer, were cast the sockets of the Sanctuary, and the sockets of the vaile: an hundred sockets of the hundred talents, a talent for a socket.",
        }),

        ("exodus", 38, 28) => Some(Verse {
            content: "And of the thousand, seuen hundred, seuentie and fiue shekels, he made hookes for the pillars, and ouerlaide their chapiters, and filleted them.",
        }),

        ("exodus", 38, 29) => Some(Verse {
            content: "And the brasse of the offring was seuentie talents, and two thousand and foure hundred shekels.",
        }),

        ("exodus", 38, 30) => Some(Verse {
            content: "And therewith he made the sockets to the doore of the Tabernacle of the Congregation, and the brasen Altar, and the brasen grate for it, and all the vessels of the Altar,",
        }),

        ("exodus", 38, 31) => Some(Verse {
            content: "And the sockets of the court round about, and the sockets of the court gate, and all the pinnes of the Tabernacle, and all the pinnes of the court round about.",
        }),

        ("exodus", 39, 1) => Some(Verse {
            content: "And of the blew, and purple, and scarlet, they made clothes of seruice, to doe seruice in the holy place, and made the holy garments for Aaron, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 2) => Some(Verse {
            content: "And he made the Ephod of gold, blew, and purple, and scarlet, and fine twined linnen.",
        }),

        ("exodus", 39, 3) => Some(Verse {
            content: "And they did beate the golde into thinne plates, and cut it into wiers, to worke it in the blew, and in the purple, and in the scarlet, and in the fine linnen, with cunning worke.",
        }),

        ("exodus", 39, 4) => Some(Verse {
            content: "They made shoulder pieces for it, to couple it together; by the two edges was it coupled together.",
        }),

        ("exodus", 39, 5) => Some(Verse {
            content: "And the curious girdle of his Ephod that was vpon it, was of the same, according to the worke thereof: of gold, blew, and purple, and scarlet, and fine twined linnen, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 6) => Some(Verse {
            content: "And they wrought Onix stones enclosed in ouches of gold, grauen as signets are grauen, with the names of the children of Israel.",
        }),

        ("exodus", 39, 7) => Some(Verse {
            content: "And hee put them on the shoulders of the Ephod, that they should be stones for a memoriall to the children of Israel, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 8) => Some(Verse {
            content: " And he made the brestplate of cunning worke, like the worke of the Ephod, of gold, blew, and purple, and scarlet, and fine twined linnen.",
        }),

        ("exodus", 39, 9) => Some(Verse {
            content: "It was foure square, they made the brestplate double: a spanne was the length therof, and a spanne the breadth thereof being doubled.",
        }),

        ("exodus", 39, 10) => Some(Verse {
            content: "And they set in it foure rowes of stones: the first row was a Sardius, a Topaz, and a Carbuncle: this was the first row.",
        }),

        ("exodus", 39, 11) => Some(Verse {
            content: "And the second row an Emeraude, a Saphire and a Diamond.",
        }),

        ("exodus", 39, 12) => Some(Verse {
            content: "And the third row a Lygure, an Agate, and an Amethist.",
        }),

        ("exodus", 39, 13) => Some(Verse {
            content: "And the fourth row, a Berill, an Onix and a Iasper: they were enclosed in ouches of gold in their inclosings.",
        }),

        ("exodus", 39, 14) => Some(Verse {
            content: "And the stones were according to the names of the children of Israel, twelue according to their names, like the ingrauings of a signet, euery one with his name, according to the twelue tribes.",
        }),

        ("exodus", 39, 15) => Some(Verse {
            content: "And they made vpon the brestplate chaines, at the ends, of wrethen worke of pure gold.",
        }),

        ("exodus", 39, 16) => Some(Verse {
            content: "And they made two ouches of gold, and two gold rings: and put the two rings in the two ends of the brestplate.",
        }),

        ("exodus", 39, 17) => Some(Verse {
            content: "And they put the two wreathen chaines of golde in the two rings on the ends of the brestplate.",
        }),

        ("exodus", 39, 18) => Some(Verse {
            content: "And the two endes of the two wreathen chaines they fastened in the two ouches, and put them on the shoulder pieces of the Ephod, before it.",
        }),

        ("exodus", 39, 19) => Some(Verse {
            content: "And they made two rings of gold, and put them on the two endes of the brest plate vpon the border of it, which was on the side of the Ephod inward.",
        }),

        ("exodus", 39, 20) => Some(Verse {
            content: "And they made two other golden rings, and put them on the two sides of the Ephod vnderneath, toward the forepart of it, ouer against the other coupling thereof, aboue the curious girdle of the Ephod.",
        }),

        ("exodus", 39, 21) => Some(Verse {
            content: "And they did bind the brest plate by his rings vnto the rings of the Ephod, with a lace of blew, that it might be aboue the curious girdle of the Ephod, and that the brest plate might not bee loosed from the Ephod, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 22) => Some(Verse {
            content: "And he made the robe of the Ephod of wouen worke, all of blew.",
        }),

        ("exodus", 39, 23) => Some(Verse {
            content: "And there was a hole in the midst of the robe as the hole of an habergeon, with a band round about the hole, that it should not rent.",
        }),

        ("exodus", 39, 24) => Some(Verse {
            content: "And they made vpon the hemmes of the robe pomegranates, of blew, and purple, and scarlet, and twined linnen.",
        }),

        ("exodus", 39, 25) => Some(Verse {
            content: "And they made belles of pure gold, and put the belles betweene the pomegranates, vpon the hemme of the robe, round about betweene the pomegranates.",
        }),

        ("exodus", 39, 26) => Some(Verse {
            content: "A bell and a pomegranate, a bell and a pomegranate round about the hemme of the robe to minister in, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 27) => Some(Verse {
            content: "And they made coats of fine linnen, of wouen worke, for Aaron and for his sonnes.",
        }),

        ("exodus", 39, 28) => Some(Verse {
            content: "And a miter of fine linnen, and goodly bonnets of fine linnen, and linnen breeches of fine twined linnen,",
        }),

        ("exodus", 39, 29) => Some(Verse {
            content: "And a girdle of fine twined linnen and blew, and purple, and scarlet of needle worke, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 30) => Some(Verse {
            content: "And they made the plate of the holy Crowne of pure gold, and wrote vpon it a writing, like to the engrauings of a signet, HOLINES TO THE LORD.",
        }),

        ("exodus", 39, 31) => Some(Verse {
            content: "And they tied vnto it a lace of blew to fasten it on high vpon the mitre, as the Lord commanded Moses.",
        }),

        ("exodus", 39, 32) => Some(Verse {
            content: "Thus was all the worke of the Tabernacle of the tent of the Congregation finished: and the children of Israel did according to al that the Lord commanded Moses, so did they.",
        }),

        ("exodus", 39, 33) => Some(Verse {
            content: "And they brought the Tabernacle vnto Moses, the tent, and all his furniture, his taches, his boards, his barres, and his pillars, and his sockets.",
        }),

        ("exodus", 39, 34) => Some(Verse {
            content: "And the couering of rammes skinnes died red, and the couering of badgers skinnes, and the vaile of the couering:",
        }),

        ("exodus", 39, 35) => Some(Verse {
            content: "The Arke of the Testimony, and the staues thereof, and the Mercie seat,",
        }),

        ("exodus", 39, 36) => Some(Verse {
            content: "The Table, and all the vessels thereof, and the Shew bread:",
        }),

        ("exodus", 39, 37) => Some(Verse {
            content: "The pure Candlesticke, with the lampes thereof, euen with the lampes to be set in order, and all the vessels thereof, and the oyle for light:",
        }),

        ("exodus", 39, 38) => Some(Verse {
            content: "And the golden altar, and the anointing oyle, and the sweet incense, and the hanging for the Tabernacle doore:",
        }),

        ("exodus", 39, 39) => Some(Verse {
            content: "The brasen altar, and his grate of brasse, his staues, and all his vessels, the lauer and his foote:",
        }),

        ("exodus", 39, 40) => Some(Verse {
            content: "The hangings of the Court, his pillars, and his sockets, and the hanging for the court gate, his coards, and his pinnes, and all the vessels of the seruice of the Tabernacle, for the tent of the Congregation:",
        }),

        ("exodus", 39, 41) => Some(Verse {
            content: "The clothes of seruice to doe seruice in the holy place, and the holy garments for Aaron the Priest, and his sonnes garments to minister in the Priests office.",
        }),

        ("exodus", 39, 42) => Some(Verse {
            content: "According to all that the Lord commanded Moses, so the children of Israel made all the worke.",
        }),

        ("exodus", 39, 43) => Some(Verse {
            content: "And Moses did looke vpon all the worke, and behold, they had done it as the Lord had commanded, euen so had they done it: and Moses blessed them.",
        }),

        ("exodus", 40, 1) => Some(Verse {
            content: "And the Lord spake vnto Moses, saying,",
        }),

        ("exodus", 40, 2) => Some(Verse {
            content: "On the first day of the first moneth shalt thou set vp the Tabernacle of the Tent of the Congregation.",
        }),

        ("exodus", 40, 3) => Some(Verse {
            content: "And thou shalt put therein the Arke of the Testimonie, and couer the Arke with the Uaile:",
        }),

        ("exodus", 40, 4) => Some(Verse {
            content: "And thou shalt bring in the Table, and set in order the things that are to be set in order vpon it, and thou shalt bring in the Candlesticke, and light the lampes thereof.",
        }),

        ("exodus", 40, 5) => Some(Verse {
            content: "And thou shalt set the Altar of gold for the incense before the Arke of the Testimonie, and put the hanging of the doore to the Tabernacle.",
        }),

        ("exodus", 40, 6) => Some(Verse {
            content: "And thou shalt set the Altar of the burnt offering, before the doore of the Tabernacle of the Tent of the Congregation.",
        }),

        ("exodus", 40, 7) => Some(Verse {
            content: "And thou shalt set the Lauer betweene the Tent of the Congregation and the Altar, and shalt put water therein.",
        }),

        ("exodus", 40, 8) => Some(Verse {
            content: "And thou shalt set vp the Court round about, and hang vp the hanging at the Court gate.",
        }),

        ("exodus", 40, 9) => Some(Verse {
            content: "And thou shalt take the annoynting oyle, and annoynt the Tabernacle and all that is therein, and shalt hallow it, and all the vessels thereof: and it shalbe holy.",
        }),

        ("exodus", 40, 10) => Some(Verse {
            content: "Aud thou shalt annoynt the Altar of the burnt offering, and all his vessels, and sanctifie the Altar: and it shalbe an Altar most Holy.",
        }),

        ("exodus", 40, 11) => Some(Verse {
            content: "And thou shalt annoynt the Lauer and his foot, and sanctifie it.",
        }),

        ("exodus", 40, 12) => Some(Verse {
            content: "And thou shalt bring Aaron and his sonnes vnto the doore of the Tabernacle of the Congregation, and wash them with water.",
        }),

        ("exodus", 40, 13) => Some(Verse {
            content: "And thou shalt put vpon Aaron the holy garments, and anoynt him, and sanctifie him, that he may minister vnto me in the Priests office.",
        }),

        ("exodus", 40, 14) => Some(Verse {
            content: "And thou shalt bring his sonnes, and clothe them with coats.",
        }),

        ("exodus", 40, 15) => Some(Verse {
            content: "And thou shalt anoynt them, as thou didst anoynt their father, that they may minister vnto mee in the Priests office: For their anoynting shall surely be an euerlasting Priesthood, throughout their generations.",
        }),

        ("exodus", 40, 16) => Some(Verse {
            content: "Thus did Moses: according to all that the Lord commanded him, so did he.",
        }),

        ("exodus", 40, 17) => Some(Verse {
            content: "And it came to passe in the first moneth, in the second yeere, on the first day of the moneth, that the Tabernacle was reared vp.",
        }),

        ("exodus", 40, 18) => Some(Verse {
            content: "And Moses reared vp the Tabernacle, and fastened his sockets, and set vp the boards thereof, and put in the barres thereof, and reared vp his pillars.",
        }),

        ("exodus", 40, 19) => Some(Verse {
            content: "And he spread abroad the tent ouer the Tabernacle, and put the couering of the Tent aboue vpon it, as the Lord commanded Moses.",
        }),

        ("exodus", 40, 20) => Some(Verse {
            content: "And he tooke and put the testimony into the Arke, and set the staues on the Arke, and put the Mercie-seat aboue vpon the Arke.",
        }),

        ("exodus", 40, 21) => Some(Verse {
            content: "And he brought the Arke into the Tabernacle, and set vp the Uaile of the couering, and couered the Arke of the Testimony, as the Lord commanded Moses.",
        }),

        ("exodus", 40, 22) => Some(Verse {
            content: "And hee put the Table in the Tent of the Congregation, vpon the side of the Tabernacle Northwaed, without the Uaile.",
        }),

        ("exodus", 40, 23) => Some(Verse {
            content: "And he set the bread in order vpon it, before the Lord, as the Lord had commanded Moses.",
        }),

        ("exodus", 40, 24) => Some(Verse {
            content: "And he put the candlesticke in the Tent of the Congregation, ouer against the Table, on the side of the Tabernacle Southward.",
        }),

        ("exodus", 40, 25) => Some(Verse {
            content: "And he lighted the lampes before the Lord, as the Lord commanded Moses.",
        }),

        ("exodus", 40, 26) => Some(Verse {
            content: "And he put the golden Altar in the Tent of the Congregation, before the Uaile.",
        }),

        ("exodus", 40, 27) => Some(Verse {
            content: "And he burnt sweet incense thereon, as the Lord commanded Moses.",
        }),

        ("exodus", 40, 28) => Some(Verse {
            content: "And hee set vp the hanging, at the doore of the Tabernacle.",
        }),

        ("exodus", 40, 29) => Some(Verse {
            content: "And he put the Altar of burnt offering by the doore of the Tabernacle of the Tent of the Congregation, and offered vpon it the burnt offering, and the meat offring, as the Lord commanded Moses.",
        }),

        ("exodus", 40, 30) => Some(Verse {
            content: "And he set the Lauer betweene the Tent of the Congregation and the Altar, & put water there, to wash withall.",
        }),

        ("exodus", 40, 31) => Some(Verse {
            content: "And Moses, and Aaron and his sonnes, washed their hands, and their feet thereat.",
        }),

        ("exodus", 40, 32) => Some(Verse {
            content: "When they went into the Tent of the Congregation, and when they came neere vnto the Altar, they washed, as the Lord commanded Moses.",
        }),

        ("exodus", 40, 33) => Some(Verse {
            content: "And hee reared vp the Court round about the Tabernacle, and the Altar, & set vp the hanging of the Court gate: so Moses finished the worke.",
        }),

        ("exodus", 40, 34) => Some(Verse {
            content: "Then a cloud couered the Tent of the Congregation, and the glory of the Lord filled the Tabernacle.",
        }),

        ("exodus", 40, 35) => Some(Verse {
            content: "And Moses was not able to enter into the Tent of the Congregation, because the cloud abode thereon, and the glory of the Lord filled the Tabernacle.",
        }),

        ("exodus", 40, 36) => Some(Verse {
            content: "And when the cloud was taken vp from ouer the Tabernacle, the children of Israel went onward in all their iourneys:",
        }),

        ("exodus", 40, 37) => Some(Verse {
            content: "But if the cloud were not taken vp, then they iourneyed not, till the day that it was taken vp.",
        }),

        ("exodus", 40, 38) => Some(Verse {
            content: "For the cloud of the Lord was vpon the Tabernacle by day, and fire was on it by night, in the sight of all the house of Israel, throughout all their iourneys.",
        }),

        ("leviticus", 1, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 1, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 2, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 3, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 4, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 5, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 6, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 7, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 8, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 9, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 10, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 39) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 40) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 41) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 42) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 43) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 44) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 45) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 46) => Some(Verse {
            content: "",
        }),

        ("leviticus", 11, 47) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 12, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 39) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 40) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 41) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 42) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 43) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 44) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 45) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 46) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 47) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 48) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 49) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 50) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 52) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 53) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 54) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 55) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 56) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 57) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 58) => Some(Verse {
            content: "",
        }),

        ("leviticus", 13, 59) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 39) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 40) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 41) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 42) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 43) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 44) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 45) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 46) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 47) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 48) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 49) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 50) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 51) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 52) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 53) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 54) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 55) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 56) => Some(Verse {
            content: "",
        }),

        ("leviticus", 14, 57) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 15, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 16, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 17, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 18, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 19, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 20, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 21, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 22, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 39) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 40) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 41) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 42) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 43) => Some(Verse {
            content: "",
        }),

        ("leviticus", 23, 44) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 24, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 39) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 40) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 41) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 42) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 43) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 44) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 45) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 46) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 47) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 48) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 49) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 50) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 51) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 52) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 53) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 54) => Some(Verse {
            content: "",
        }),

        ("leviticus", 25, 55) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 34) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 35) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 36) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 37) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 38) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 39) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 40) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 41) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 42) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 43) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 44) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 45) => Some(Verse {
            content: "",
        }),

        ("leviticus", 26, 46) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 1) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 2) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 3) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 4) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 5) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 6) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 7) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 8) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 9) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 10) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 11) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 12) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 13) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 14) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 15) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 16) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 17) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 18) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 19) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 20) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 21) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 22) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 23) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 24) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 25) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 26) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 27) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 28) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 29) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 30) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 31) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 32) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 33) => Some(Verse {
            content: "",
        }),

        ("leviticus", 27, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 51) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 52) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 53) => Some(Verse {
            content: "",
        }),

        ("numbers", 1, 54) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 2, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 20) => Some(Verse {
            content: "",
        }),
        
        ("numbers", 3, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 3, 51) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 4, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 5, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 6, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 51) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 52) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 53) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 54) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 55) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 56) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 57) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 58) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 59) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 60) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 61) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 62) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 63) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 64) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 65) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 66) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 67) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 68) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 69) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 70) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 71) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 72) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 73) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 74) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 75) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 76) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 77) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 78) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 79) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 80) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 81) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 82) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 83) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 84) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 85) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 86) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 87) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 88) => Some(Verse {
            content: "",
        }),

        ("numbers", 7, 89) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 8, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 9, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 10, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 11, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 12, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 13, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 14, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 15, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 16, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 17, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 18, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 19, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 20, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 21, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 22, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 23, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 24, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 25, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 51) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 52) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 53) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 54) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 55) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 56) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 57) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 58) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 59) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 60) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 61) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 62) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 63) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 64) => Some(Verse {
            content: "",
        }),

        ("numbers", 26, 65) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 27, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 28, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 29, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 30, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 51) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 52) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 53) => Some(Verse {
            content: "",
        }),

        ("numbers", 31, 54) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 32, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 35) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 36) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 37) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 38) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 39) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 40) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 41) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 42) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 43) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 44) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 45) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 46) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 47) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 48) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 49) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 50) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 51) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 52) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 53) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 54) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 55) => Some(Verse {
            content: "",
        }),

        ("numbers", 33, 56) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 34, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 14) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 15) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 16) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 17) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 18) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 19) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 20) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 21) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 22) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 23) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 24) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 25) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 26) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 27) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 28) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 29) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 30) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 31) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 32) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 33) => Some(Verse {
            content: "",
        }),

        ("numbers", 35, 34) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 1) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 2) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 3) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 4) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 5) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 6) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 7) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 8) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 9) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 10) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 11) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 12) => Some(Verse {
            content: "",
        }),

        ("numbers", 36, 13) => Some(Verse {
            content: "",
        }),

        ("numbers", 37, 1) => Some(Verse {
            content: "",
        }),

        







        








       
































        



        












      






        













        


        




        _ => None,
    }
}
