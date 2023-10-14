/*
 * endpoints/page.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use crate::models::page::Model as PageModel;
use crate::models::page_revision::Model as PageRevisionModel;
use crate::services::page::{
    CreatePage, CreatePageOutput, DeletePage, EditPage, EditPageOutput, GetPage,
    GetPageDirect, GetPageOutput, MovePage, RestorePage, RollbackPage,
};
use crate::services::{Result, TextService};
use crate::web::{PageDetails, Reference};

pub async fn page_create(
    ctx: ServiceContext<'_>,
    params: Params<'static>,
) -> Result<CreatePageOutput> {
    let input: CreatePage = params.parse()?;
    tide::log::info!("Creating new page in site ID {}", input.site_id);
    PageService::create(&ctx, input).await
}

pub async fn page_get(
    ctx: ServiceContext<'_>,
    params: Params<'static>,
) -> Result<GetPageOutput> {
    let GetPage {
        site_id,
        page: reference,
        details,
    } = params.parse()?;

    tide::log::info!("Getting page {reference:?} in site ID {site_id}");
    let page = PageService::get(&ctx, site_id, reference).await?;
    let revision = PageRevisionService::get_latest(&ctx, site_id, page.page_id).await?;
    build_page_output(&ctx, page, revision, details).await
}

pub async fn page_get_direct(
    ctx: ServiceContext<'_>,
    params: Params<'static>,
) -> Result<GetPageOutput> {
    let GetPageDirect {
        site_id,
        page_id,
        details,
    } = params.parse()?;

    tide::log::info!("Getting page ID {page_id} in site ID {site_id}");
    let page = PageService::get_direct(&ctx, site_id, page_id).await?;
    let revision = PageRevisionService::get_latest(&ctx, site_id, page_id).await?;
    build_page_output(&ctx, page, revision, details).await
}

pub async fn page_edit(
    ctx: ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Option<EditPageOutput>> {
    let input: EditPage = params.parse()?;
    tide::log::info!("Editing page {:?} in site ID {}", input.page, input.site_id);
    PageService::edit(&ctx, input).await
}

pub async fn page_delete(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::from_req(&req, &txn);

    let input: DeletePage = req.body_json().await?;
    tide::log::info!(
        "Deleting page {:?} in site ID {}",
        input.page,
        input.site_id,
    );

    let output = PageService::delete(&ctx, input).await?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn page_move(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::from_req(&req, &txn);

    let input: MovePage = req.body_json().await?;
    tide::log::info!(
        "Moving page {:?} in site ID {} to {}",
        input.page,
        input.site_id,
        input.new_slug,
    );

    let output = PageService::r#move(&ctx, input).await?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn page_rerender(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::from_req(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let page_id = req.param("page_id")?.parse()?;
    tide::log::info!("Re-rendering page ID {page_id} in site ID {site_id}");

    PageRevisionService::rerender(&ctx, site_id, page_id).await?;

    txn.commit().await?;
    Ok(Response::new(StatusCode::NoContent))
}

pub async fn page_restore(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::from_req(&req, &txn);

    let input: RestorePage = req.body_json().await?;
    tide::log::info!(
        "Un-deleting page ID {} in site ID {}",
        input.page_id,
        input.site_id,
    );

    let output = PageService::restore(&ctx, input).await?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn page_rollback(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::from_req(&req, &txn);

    let input: RollbackPage = req.body_json().await?;
    tide::log::info!(
        "Rolling back page {:?} in site ID {} to revision number {}",
        input.page,
        input.site_id,
        input.revision_number,
    );

    let output = PageService::rollback(&ctx, input).await?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

async fn build_page_output(
    ctx: &ServiceContext<'_>,
    page: PageModel,
    revision: PageRevisionModel,
    details: PageDetails,
) -> Result<GetPageOutput> {
    // Get category slug from ID
    let category =
        CategoryService::get(ctx, page.site_id, Reference::from(page.page_category_id))
            .await?;

    // Get text data, if requested
    let (wikitext, compiled_html) = try_join!(
        TextService::get_maybe(ctx, details.wikitext, &revision.wikitext_hash),
        TextService::get_maybe(ctx, details.compiled_html, &revision.compiled_hash),
    )?;

    // Calculate score
    let rating = ScoreService::score(ctx, page.page_id).await?;

    // Build result struct
    Ok(GetPageOutput {
        page_id: page.page_id,
        page_created_at: page.created_at,
        page_updated_at: page.updated_at,
        page_deleted_at: page.deleted_at,
        page_revision_count: revision.revision_number + 1,
        site_id: page.site_id,
        page_category_id: category.category_id,
        page_category_slug: category.slug,
        discussion_thread_id: page.discussion_thread_id,
        revision_id: revision.revision_id,
        revision_type: revision.revision_type,
        revision_created_at: revision.created_at,
        revision_number: revision.revision_number,
        revision_user_id: revision.user_id,
        wikitext,
        compiled_html,
        compiled_at: revision.compiled_at,
        compiled_generator: revision.compiled_generator,
        revision_comments: revision.comments,
        hidden_fields: revision.hidden,
        title: revision.title,
        alt_title: revision.alt_title,
        slug: revision.slug,
        tags: revision.tags,
        rating,
    })
}
