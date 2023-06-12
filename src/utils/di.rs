use crate::appv2::features::article::presenters::ArticlePresenter;
use crate::appv2::features::article::repositories::ArticleRepository;
use crate::appv2::features::article::usecases::ArticleUsecase;
use crate::appv2::features::favorite::{
    presenters::FavoritePresenter, repositories::FavoriteRepository, usecases::FavoriteUsecase,
};
use crate::appv2::features::profile::{
    presenters::ProfilePresenter, repositories::ProfileRepository, usecases::ProfileUsecase,
};
use crate::appv2::features::tag::presenters::TagPresenter;
use crate::appv2::features::tag::repositories::TagRepository;
use crate::appv2::features::tag::usecases::TagUsecase;
use crate::appv2::features::user::{
    presenters::UserPresenter, repositories::UserRepository, usecases::UserUsecase,
};

use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UserRepository,
    pub user_usecase: UserUsecase,
    pub user_presenter: UserPresenter,

    /**
     * Profile
     */
    pub profile_repository: ProfileRepository,
    pub profile_presenter: ProfilePresenter,
    pub profile_usecase: ProfileUsecase,

    /**
     * Favorite
     */
    pub favorite_repository: FavoriteRepository,
    pub favorite_presenter: FavoritePresenter,
    pub favorite_usecase: FavoriteUsecase,

    /**
     * Article
     */
    pub article_repository: ArticleRepository,
    pub article_presenter: ArticlePresenter,
    pub article_usecase: ArticleUsecase,

    /**
     * Tag
     */
    pub tag_repository: TagRepository,
    pub tag_presenter: TagPresenter,
    pub tag_usecase: TagUsecase,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let user_repository = UserRepository::new(pool.clone());
        let profile_repository = ProfileRepository::new(pool.clone());
        let favorite_repository = FavoriteRepository::new(pool.clone());
        let article_repository = ArticleRepository::new(pool.clone());
        let tag_repository = TagRepository::new(pool.clone());

        // Presenter
        let user_presenter = UserPresenter::new();
        let profile_presenter = ProfilePresenter::new();
        let favorite_presenter = FavoritePresenter::new();
        let article_presenter = ArticlePresenter::new();
        let tag_presenter = TagPresenter::new();

        // Usecase
        let user_usecase = UserUsecase::new(user_repository.clone(), user_presenter.clone());
        let profile_usecase = ProfileUsecase::new(
            (profile_repository.clone(), user_repository.clone()),
            profile_presenter.clone(),
        );
        let favorite_usecase = FavoriteUsecase::new(
            favorite_repository.clone(),
            favorite_presenter.clone(),
            article_repository.clone(),
        );
        let article_usecase =
            ArticleUsecase::new(article_repository.clone(), article_presenter.clone());
        let tag_usecase = TagUsecase::new(tag_repository.clone(), tag_presenter.clone());

        Self {
            // User
            user_repository,
            user_usecase,
            user_presenter,

            // Profile
            profile_presenter,
            profile_repository,
            profile_usecase,

            // Favorite
            favorite_repository,
            favorite_presenter,
            favorite_usecase,

            // Article
            article_repository,
            article_presenter,
            article_usecase,

            // Tag
            tag_repository,
            tag_presenter,
            tag_usecase,
        }
    }
}
