use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Wish {
    pub id: i32,
    pub title: String,
    pub deadline: Option<NaiveDate>,
    pub priority: i32, // 優先度、聞くときはやらなきゃいけないことかやりたいことか着たほうがいい
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub root_id: i32, // 紐づく Wish の ID
    pub title: String,

    // --- 行動の解像度（IPOモデル） ---
    pub input: Option<String>, // 準備
    pub action: String,        // 具体的な手順（1〜2分でできる最小単位）
    pub output: String,        // 終わり・完了条件

    // --- 無気力・先延ばし対策（行動科学の要素） ---
    pub not_to_do: Option<String>, // やらないこと,ここまではやらなくていい、やったら行けない妨げ
    pub scheduled_at: Option<String>, // いつやるか

    // --- 重みと状態 ---
    pub weight: i32, // 難易度・獲得スコア（1〜3推奨）
}

#[derive(Clone, Debug)]
pub struct DoneTask {
    pub id: i32,
    pub root_id: i32,
    pub title: String,
    pub completed_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct WishBlock {
    pub wish: Wish,
    pub tasks: Vec<Task>,
}
