use rollover::{collect_all, pages, ListOptions, Page, RolloverError};

fn mock_list_fn(
    data: Vec<Vec<String>>,
    total: i64,
) -> impl FnMut(ListOptions) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Page<String>, RolloverError>> + Send>>
{
    let data = std::sync::Arc::new(data);
    move |opts: ListOptions| {
        let data = data.clone();
        Box::pin(async move {
            let idx = if opts.limit > 0 {
                (opts.offset / opts.limit) as usize
            } else {
                0
            };
            if idx >= data.len() {
                return Ok(Page {
                    data: vec![],
                    total,
                    limit: opts.limit,
                    offset: opts.offset,
                });
            }
            Ok(Page {
                data: data[idx].clone(),
                total,
                limit: opts.limit,
                offset: opts.offset,
            })
        })
    }
}

#[tokio::test]
async fn test_pages_iterates_all() {
    let mut iter = pages(
        mock_list_fn(
            vec![
                vec!["a".into(), "b".into()],
                vec!["c".into(), "d".into()],
                vec!["e".into()],
            ],
            5,
        ),
        Some(ListOptions {
            limit: 2,
            ..Default::default()
        }),
    );

    let mut all = Vec::new();
    while iter.next().await {
        if let Some(page) = iter.page() {
            all.extend(page.data.clone());
        }
    }
    assert!(iter.err().is_none());
    assert_eq!(all.len(), 5);
}

#[tokio::test]
async fn test_pages_stops_on_empty() {
    let mut iter = pages(mock_list_fn(vec![vec![]], 0), None);
    assert!(!iter.next().await);
}

#[tokio::test]
async fn test_pages_stops_on_short_page() {
    let mut iter = pages(
        move |_opts: ListOptions| {
            Box::pin(async move {
                Ok(Page {
                    data: vec!["a".to_string()],
                    total: 1,
                    limit: 100,
                    offset: 0,
                })
            })
                as std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<Page<String>, RolloverError>> + Send>,
                >
        },
        None,
    );

    assert!(iter.next().await);
    assert!(!iter.next().await);
}

#[tokio::test]
async fn test_pages_default_limit() {
    let mut iter = pages(
        |opts: ListOptions| {
            Box::pin(async move {
                assert_eq!(opts.limit, 100);
                Ok(Page::<String> {
                    data: vec![],
                    total: 0,
                    limit: opts.limit,
                    offset: 0,
                })
            })
                as std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<Page<String>, RolloverError>> + Send>,
                >
        },
        None,
    );
    iter.next().await;
}

#[tokio::test]
async fn test_collect_all() {
    let all = collect_all(
        mock_list_fn(
            vec![vec!["a".into(), "b".into()], vec!["c".into()]],
            3,
        ),
        Some(ListOptions {
            limit: 2,
            ..Default::default()
        }),
    )
    .await
    .unwrap();

    assert_eq!(all.len(), 3);
}

#[tokio::test]
async fn test_collect_empty() {
    let all = collect_all(mock_list_fn(vec![vec![]], 0), None)
        .await
        .unwrap();

    assert_eq!(all.len(), 0);
}
