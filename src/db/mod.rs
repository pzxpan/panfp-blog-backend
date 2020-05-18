use crate::models::{user::{User, NewUser, ChangePassword},
                    article::Article,
                    article::ArticleDetail,
                    article::UserArticleDetail,
                    article::Id,
                    label::Label,
                    comment::{Comment, NewComment, DisplayCommentInfo},
                    category::Category,
                    category::HotCategory,
                    app_state::AppState,
                    result_response::ResultResponse};
use crate::models::errors::{AppError, AppErrorCode::*};

use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use actix_web::web;
use uuid::Uuid;
use crate::models::article::NewArticle;
use crate::models::image::Image;
use crate::models::user::{UserDetailInfo, UserDisplayInfo};

pub async fn get_articles(client: &Client, article_type: i32) -> Result<Vec<Article>, AppError> {
    let single_category = "select article_id,user_id,view_count,title,comment_count,like_count,date,intro from public.\"article\" WHERE EXISTS ( SELECT * FROM public.\"article_category\" WHERE public.\"article\".article_id = public.\"article_category\".article_id
    AND public.\"article_category\".category_id = $1) order by view_count desc";
    let multi_category = "select article_id,user_id,view_count,title,comment_count,like_count,date,intro from public.\"article\" WHERE EXISTS ( SELECT * FROM public.\"article_category\" WHERE public.\"article\".article_id = public.\"article_category\".article_id
    AND public.\"article_category\".category_id > $1  AND public.\"article_category\".category_id < $2) order by view_count desc";
    let statement;
    if article_type < 10 {
        statement = client
            .prepare(multi_category)
            .await?;
        let articles = client
            .query(&statement, &[&(article_type * 10), &((article_type + 1) * 10)])
            .await?
            .iter()
            .map(|row| { Article::from_row_ref(row).unwrap() })
            .collect::<Vec<Article>>();
        Ok(articles)
    } else {
        statement = client
            .prepare(single_category)
            .await?;
        let articles = client
            .query(&statement, &[&article_type])
            .await?
            .iter()
            .map(|row| { Article::from_row_ref(row).unwrap() })
            .collect::<Vec<Article>>();
        Ok(articles)
    }
}

pub async fn get_hot_articles(client: &Client, article_type: i32) -> Result<Vec<Article>, AppError> {
    let multi_statment = client
        .prepare("select article_id,user_id,view_count,title,comment_count,like_count,date,intro from public.\"article\" WHERE EXISTS ( SELECT * FROM public.\"article_category\" WHERE public.\"article\".article_id = public.\"article_category\".article_id
    AND public.\"article_category\".category_id > $1 and public.\"article_category\".category_id < $2) order by view_count desc limit 5")
        .await?;

    let articles = client
        .query(&multi_statment, &[&(article_type * 10), &((article_type + 1) * 10)])
        .await?
        .iter()
        .map(|row| { Article::from_row_ref(row).unwrap() })
        .collect::<Vec<Article>>();
    Ok(articles)
}

pub async fn get_user_articles(client: &Client, user_id: i32) -> Result<Vec<Article>, AppError> {
    let statement = client
        .prepare("select article_id,user_id,view_count,title,comment_count,like_count,date,intro from public.\"article\" WHERE user_id = $1 order by view_count desc")
        .await?;

    let articles = client
        .query(&statement, &[&user_id])
        .await?
        .iter()
        .map(|row| { Article::from_row_ref(row).unwrap() })
        .collect::<Vec<Article>>();
    Ok(articles)
}

pub async fn article_detail(client: &Client, article_id: i32) -> Result<ArticleDetail, AppError> {
    let statement = client
        .prepare("select article_id,user_id,view_count,title,comment_count,like_count,date,content_html,intro from public.\"article\" \
        where article_id = $1")
        .await?;

    client
        .query(&statement, &[&article_id])
        .await?
        .iter()
        .map(|row| { ArticleDetail::from_row_ref(row).unwrap() })
        .collect::<Vec<ArticleDetail>>()
        .pop()
        .ok_or(AppError {
            message: "找不到相应的文章".to_string(),
            err_code: DbNotFoundErr,
        })
}

pub async fn article_comment_list(client: &Client, article_id: i32) -> Result<Vec<Comment>, AppError> {
    let statement = client
        .prepare("select * from public.\"comment\" where article_id = $1")
        .await?;

    let comments = client
        .query(&statement, &[&article_id])
        .await?
        .iter()
        .map(|row| { Comment::from_row_ref(row).unwrap() })
        .collect::<Vec<Comment>>();
    Ok(comments)
}

pub async fn user_article_comment_list(client: &Client, article_id: i32) -> Result<Vec<DisplayCommentInfo>, AppError> {
    let statement = client
        .prepare("SELECT comment_id,article_id,user_id,content,date,public.\"user\".nick_name from  public.\"comment\" left join public.\"user\" using(user_id)
where article_id = $1")
        .await?;

    let comments = client
        .query(&statement, &[&article_id])
        .await?
        .iter()
        .map(|row| DisplayCommentInfo::from(row))
        .collect::<Vec<DisplayCommentInfo>>();
    Ok(comments)
}

pub async fn article_labels(client: &Client, article_id: i32) -> Result<Vec<Label>, AppError> {
    // SELECT article_id,
    // ARRAY(SELECT unnest(array_agg(label_id))) AS lables
    // FROM  public.article_label
    // GROUP BY  article_id;
    let statement = client
        .prepare("select *  from public.\"label\" WHERE EXISTS ( SELECT * FROM public.\"article_label\" WHERE public.\"label\".label_id = public.\"article_label\".label_id
AND public.\"article_label\".article_id = $1)")
        .await?;

    let labels = client
        .query(&statement, &[&article_id])
        .await?
        .iter()
        .map(|row| { Label::from_row_ref(row).unwrap() })
        .collect::<Vec<Label>>();
    Ok(labels)
}

pub async fn all_labels(client: &Client) -> Result<Vec<Label>, AppError> {
    let statement = client
        .prepare("select *  from public.\"label\"")
        .await?;

    let labels = client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| { Label::from_row_ref(row).unwrap() })
        .collect::<Vec<Label>>();
    Ok(labels)
}

pub async fn categories(client: &Client, category_id: i32) -> Result<Vec<Category>, AppError> {
    let statement = client
        .prepare("select * from public.\"category\" where parent_id = $1")
        .await?;

    let categories = client
        .query(&statement, &[&category_id])
        .await?
        .iter()
        .map(|row| { Category::from_row_ref(row).unwrap() })
        .collect::<Vec<Category>>();
    Ok(categories)
}

pub async fn hot_categories(client: &Client) -> Result<Vec<HotCategory>, AppError> {
    let statement = client
        .prepare("select * from public.\"hot_category\" ")
        .await?;

    let categories = client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| { HotCategory::from_row_ref(row).unwrap() })
        .collect::<Vec<HotCategory>>();
    Ok(categories)
}


pub async fn is_like(client: &Client, user_id: i32, article_id: i32) -> Result<i32, AppError> {
    let statement = client
        .prepare("select * from public.article_like where article_id=$1 and user_id=$2")
        .await?;
    let result = client
        .query(&statement, &[&article_id, &user_id])
        .await?;
    if result.len() > 0 {
        Ok(1)
    } else {
        Ok(0)
    }
}

pub async fn add_like(client: &Client, user_id: i32, article_id: i32) -> Result<i32, AppError> {
    let statement = client
        .prepare("insert into public.article_like (user_id, article_id) VALUES ($1,$2) returning id")
        .await?;
    let add = client
        .prepare("update public.\"article\" set like_count = like_count + 1 where article_id = $1 returning like_count")
        .await?;

    let result = client
        .query(&statement, &[&user_id, &article_id])
        .await?;

    if result.len() > 0 {
        client
            .query(&add, &[&article_id])
            .await?
            .iter()
            .map(|row| {
                let like_count: i32 = row.get("like_count");
                like_count
            })
            .collect::<Vec<i32>>()
            .pop()
            .ok_or(AppError {
                message: "点赞失败".to_string(),
                err_code: DbNotFoundErr,
            })
    } else {
        Err(AppError {
            message: "点赞失败".to_string(),
            err_code: DbNotFoundErr,
        })
    }
}

pub async fn add_image(client: &Client, user_id: i32, path: String, source_name: String) -> Result<Image, AppError> {
    let query_str = format!("insert into public.\"image\" (path, source_name,user_id) VALUES ('{}','{}',{}) returning id,user_id,source_name,path,create_time",
                            &path, &source_name, &user_id);
    let statement = client
        .prepare(query_str.as_ref())
        .await?;

    client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| {
            Image::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Image>>()
        .pop()
        .ok_or(AppError {
            message: "添加图片失败".to_string(),
            err_code: DbNotFoundErr,
        })
}

pub async fn delete_image(client: &Client, user_id: i32, image_id: i32) -> Result<i32, AppError> {
    let statement = client
        .prepare("delete from public.\"image\"  where user_id=$1 and id = $2 returning id")
        .await?;
    client
        .query(&statement, &[&user_id, &image_id])
        .await?
        .iter()
        .map(|row| {
            let id: i32 = row.get("id");
            id
        })
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "删除图片失败".to_string(),
            err_code: DbNotFoundErr,
        })
}

pub async fn get_user_image(client: &Client, user_id:i32) -> Result<Vec<Image>, AppError> {
    let statement = client
        .prepare("select *  from public.\"image\"  where user_id=$1")
        .await?;
    let result =  client
        .query(&statement, &[&user_id])
        .await?
        .iter()
        .map(|row| {
            Image::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Image>>();
    Ok(result)
}

pub async fn cancel_like(client: &Client, user_id: i32, article_id: i32) -> Result<i32, AppError> {
    let del = client
        .prepare("delete from public.article_like where user_id=$1 and article_id=$2 returning id")
        .await?;
    let del_cnt = client
        .prepare("update public.article set like_count = like_count - 1 where article_id = $1 returning like_count")
        .await?;
    let result = client
        .query(&del, &[&user_id, &article_id])
        .await?;

    if result.len() > 0 {
        client
            .query(&del_cnt, &[&article_id])
            .await?
            .iter()
            .map(|row| {
                let like_count: i32 = row.get("like_count");
                like_count
            })
            .collect::<Vec<i32>>()
            .pop()
            .ok_or(AppError {
                message: "取消用户点赞失败".to_string(),
                err_code: DbNotFoundErr,
            })
    } else {
        Err(AppError {
            message: "取消用户点赞失败".to_string(),
            err_code: DbNotFoundErr,
        })
    }
}

pub async fn add_view_cnt(client: &Client, article_id: i32) -> Result<i32, AppError> {
    let statement = client
        .prepare("update public.\"article\" set view_count = view_count + 1 where article_id = $1")
        .await?;
    client
        .query(&statement, &[&article_id])
        .await?;
    Ok(0)
}


pub async fn add_aritcle_comment(client: &Client, comment: web::Json<NewComment>) -> Result<i32, AppError> {
    let statement = client
        .prepare("insert into public.\"comment\" (user_id, article_id, content) \
                         values ($1,$2,$3) returning comment_id")
        .await?;
    let result = client
        .query(&statement, &[&comment.user_id, &comment.article_id, &comment.content])
        .await?;
    if result.len() > 0 {
        let statement = client
            .prepare("update public.\"article\" set comment_count = comment_count + 1 where article_id = $1 returning comment_count")
            .await?;
        client
            .query(&statement, &[&comment.article_id])
            .await?
            .iter()
            .map(|row| {
                let comment_count: i32 = row.get("comment_count");
                comment_count
            })
            .collect::<Vec<i32>>()
            .pop()
            .ok_or(AppError {
                message: "评论添加失败".to_string(),
                err_code: DbNotFoundErr,
            })
    } else {
        Err(AppError {
            message: "评论添加失败".to_string(),
            err_code: DbNotFoundErr,
        })
    }
}

pub async fn update_article(client: &Client, article: &web::Json<NewArticle>) -> Result<i32, AppError> {
    if let article_id = Some(article.article_id) {
        let query_str = format!("select update_article_detail({},{},'{}','{}','{}',{},array{:?})",
                                article.article_id.unwrap(), article.user_id, article.title, article.intro, article.content_html, article.category_id, article.labels);
        let statement = client
            .prepare(query_str.as_ref())
            .await?;
        client
            .query(&statement, &[])
            .await?
            .iter()
            .map(|row| row.get("update_article_detail"))
            .collect::<Vec<i32>>()
            .pop()
            .ok_or(AppError {
                message: "修改文章失败".to_string(),
                err_code: DbNotFoundErr,
            })
    } else {
        Err(AppError {
            message: "缺少文章Id失败".to_string(),
            err_code: DbNotFoundErr,
        })
    }
}

pub async fn get_user_article(client: &Client, id: &web::Json<Id>) -> Result<UserArticleDetail, AppError> {
    let statement = client
        .prepare("select article_id,user_id,view_count,title,comment_count,intro,like_count,date,content_html, category_id from public.\"article\" left join article_category using(article_id) \
        where article_id = $1 and user_id = $2")
        .await?;

    client
        .query(&statement, &[&id.article_id, &id.user_id])
        .await?
        .iter()
        .map(|row| { UserArticleDetail::from_row_ref(row).unwrap() })
        .collect::<Vec<UserArticleDetail>>()
        .pop()
        .ok_or(AppError {
            message: "找不到相应的文章".to_string(),
            err_code: DbNotFoundErr,
        })
}

pub async fn add_aritcle(client: &Client, article: &web::Json<NewArticle>) -> Result<i32, AppError> {
    let query_str = format!("select insert_article_detail({},'{}','{}','{}',{},array{:?})",
                            article.user_id, article.title, article.intro, article.content_html, article.category_id, article.labels);
    // let statement = client
    //     .prepare("select insert_article_detail(13,'ffffggggg','aaaabbbbb','<p>dddddadf</p>',11,array[1,2,3])")
    //     .await?;
    let statement = client
        .prepare(query_str.as_ref())
        .await?;
    client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| row.get("insert_article_detail"))
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "上传文章失败".to_string(),
            err_code: DbNotFoundErr,
        })
}

// pub async fn add_aritcle(client: &Client, article: &web::Json<NewArticle>) -> Result<i32, AppError> {
//     let statement = client
//         .prepare("insert into public.\"article\" (user_id, title, intro,content_html) \
//                          values ($1,$2,$3,$4) returning article_id")
//         .await?;
//     client
//         .query(&statement, &[&article.user_id, &article.title, &article.intro, &article.content_html])
//         .await?
//         .iter()
//         .map(|row| row.get("article_id"))
//         .collect::<Vec<i32>>()
//         .pop()
//         .ok_or(AppError {
//             message: "上传文章失败".to_string(),
//             err_code: DbNotFoundErr,
//         })
// }
// declare
// a_id integer;
// BEGIN
// insert into public.article (user_id, title, intro,content_html) values (u_id,a_title,a_intro,a_content_html);
// a_id = (select MAX(article_id) from public.article);
// FOR a_lable_id IN labels loop
// INSERT INTO public.article_label (article_id, label_id) VALUES (a_id,a_label_id);
// Insert into public.article_category (article_id,category_id) VALUES(a_id,a_category_id);
// RETURN a_id;
// END;
// for a_label_id in a_labels loop
// INSERT INTO public.article_label (article_id, label_id) VALUES (a_id,a_lable_id);
// end loop;
//select insert_article_detail(13,'ggggg','bbbbb','<p>dadf</p>',11,array[1,2,3])
//select insert_article_detail(13,'ggggg','bbbbb','<p>dadf</p>',11,array[1,2,3])
pub async fn add_article_label(client: &Client, article_id: i32, label_ids: Vec<i32>) -> Result<i32, AppError> {
    //批量插入
    let mut query_str = "INSERT INTO public.article_label (article_id, label_id) VALUES ".to_string();
    query_str = label_ids.iter().fold(query_str, |query_str, x| { query_str + format!("( {}, {}),", article_id, x).as_ref() });
    query_str = query_str[0..query_str.len() - 1].to_string() + " returning id";
    let statement = client
        .prepare(&query_str)
        .await?;
    client.query(&statement, &[])
        .await?
        .iter()
        .map(|row| row.get("id"))
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "插入文章标签失败".to_string(),
            err_code: DbNotFoundErr,
        })
}

pub async fn del_aritcle_comment(client: &Client, user_id: i32, article_id: i32, comment_id: i32) -> Result<i32, AppError> {
    let del = client
        .prepare("delete from public.comment where user_id=$1 and article_id=$2 comment_id=$3 returning comment_id")
        .await?;
    let del_cnt = client
        .prepare("update public.\"article\" set comment_count = comment_count - 1 where article_id = $1 returning article_id")
        .await?;
    let result = client
        .query(&del, &[&user_id, &article_id, &comment_id])
        .await?;
    if result.len() > 0 {
        client
            .query(&del_cnt, &[&article_id])
            .await?
            .iter()
            .map(|row| row.get("article_id"))
            .collect::<Vec<i32>>()
            .pop()
            .ok_or(AppError {
                message: "删除评论失败".to_string(),
                err_code: DbNotFoundErr,
            })
    } else {
        Err(AppError {
            message: "删除评论失败".to_string(),
            err_code: DbNotFoundErr,
        })
    }
}

pub async fn register(client: &Client, user: &web::Json<NewUser>) -> Result<i32, AppError> {
    let statement = client
        .prepare("insert into public.\"user\" (password, email, nick_name, profession, avatar,login_session,level) \
                         values ($1,$2,$3,$4,$5,'',1) returning user_id")
        .await?;
    client
        .query(&statement, &[&user.password, &user.email, &user.nick_name, &user.profession, &user.level, &user.avatar])
        .await?
        .iter()
        .map(|row| row.get("user_id"))
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "用户注册失败".to_string(),
            err_code: DbNotFoundErr,
        })
}

pub async fn login(client: &Client, user: &web::Json<NewUser>) -> Result<User, AppError> {
    let statement = client
        .prepare("select * from public.\"user\" where email=$1 and password=$2")
        .await?;
    let list = client
        .query(&statement, &[&user.email, &user.password])
        .await?;

    if list.len() < 1 {
        Err(AppError {
            message: "用户名或密码错误".to_string(),
            err_code: AuthErr,
        })
    } else {
        list.iter()
            .map(|row| { User::from_row_ref(row).unwrap() })
            .collect::<Vec<User>>()
            .pop()
            .ok_or(AppError {
                message: "用户名或密码错误".to_string(),
                err_code: AuthErr,
            })
    }
}

pub async fn detail_user(client: &Client, user_id: i32) -> Result<User, AppError> {
    let statement = client
        .prepare("select * from public.\"user\" where user_id=$1")
        .await?;
    client
        .query(&statement, &[&user_id])
        .await?
        .iter()
        .map(|row| { User::from_row_ref(row).unwrap() })
        .collect::<Vec<User>>()
        .pop()
        .ok_or(AppError {
            message: "无相关用户信息".to_string(),
            err_code: AuthErr,
        })
}

pub async fn update_detail_user(client: &Client, user: &web::Json<UserDetailInfo>) -> Result<i32, AppError> {
    let statement = client
        .prepare("update  public.\"user\" set nick_name=$1,profession=$2, avatar=$3 where user_id= $4 returning  user_id")
        .await?;
    client
        .query(&statement, &[&user.nick_name.as_ref(), &user.profession.as_ref(), &user.avatar.as_ref(), &user.user_id.as_ref()])
        .await?
        .iter()
        .map(|row| {
            let id: i32 = row.get("user_id");
            id
        })
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "更新用户信息失败".to_string(),
            err_code: AuthErr,
        })
}

pub async fn change_password(client: &Client, user: &web::Json<ChangePassword>) -> Result<i32, AppError> {
    let statement = client
        .prepare("update  public.\"user\" set password=$1  where user_id= $2 and password=$3 returning user_id")
        .await?;
    client
        .query(&statement, &[&user.new_password, &user.user_id, &user.password])
        .await?
        .iter()
        .map(|row| {
            let id: i32 = row.get("user_id");
            id
        })
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "更改密码失败".to_string(),
            err_code: AuthErr,
        })
}

pub async fn logout(client: &Client, user_id: i32) -> Result<bool, AppError> {
    let statement = client
        .prepare("update public.\"user\" set login_session = '' where user_id=$1")
        .await?;
    let result = client.query(&statement, &[&user_id])
        .await?;
    if result.is_empty() {
        Ok(true)
    } else {
        Err(AppError {
            message: "".to_string(),
            err_code: Normal,
        })
    }
}

pub fn generate_login_session() -> String {
    Uuid::new_v4().to_simple().to_string()
}

pub async fn update_login_session_to_db(client: &Client, un: &str, login_session_str: &str) -> Result<bool, AppError> {
    let statement = client
        .prepare("update  public.\"user\" set login_session = $1 where email=$2")
        // .prepare("select * from public.\"user\" where email=$1 and password=$2")
        .await?;
    let result = client.query(&statement, &[&login_session_str, &un])
        .await;
    if result.is_ok() {
        Ok(true)
    } else {
        Err(AppError {
            message: "插入用户session数据出错".to_string(),
            err_code: DbNotFoundErr,
        })
    }
}

pub async fn verify_token(client: &Client, email: String, login_session: String) -> Result<i32, AppError> {
    let statement = client
        .prepare("select user_id from public.\"user\" where email=$1 and login_session=$2")
        .await?;
    client.query(&statement, &[&email, &login_session])
        .await?
        .iter()
        .map(|row| {
            row.get("user_id")
        })
        .collect::<Vec<i32>>()
        .pop()
        .ok_or(AppError {
            message: "没有访问权限".to_string(),
            err_code: AuthErr,
        })
}
