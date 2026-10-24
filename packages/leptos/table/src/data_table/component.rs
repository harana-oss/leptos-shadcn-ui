use super::types::*;
use leptos::prelude::*;

/// Main DataTable component
#[component]
pub fn DataTable(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] data: MaybeProp<Vec<DataRow>>,
    #[prop(into, optional)] columns: MaybeProp<Vec<DataColumn>>,
    #[prop(into, optional)] loading: MaybeProp<bool>,
    #[prop(into, optional)] error: MaybeProp<Option<String>>,
    #[prop(into, optional)] on_row_click: Option<Callback<DataRow>>,
    #[prop(into, optional)] on_row_select: Option<Callback<Vec<i32>>>,
    #[prop(into, optional)] on_sort: Option<Callback<SortConfig>>,
    #[prop(into, optional)] on_filter: Option<Callback<Vec<FilterConfig>>>,
    #[prop(into, optional)] on_export: Option<Callback<ExportFormat>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Filtering is driven by `state.filters`; there is no filter UI yet to invoke this callback.
    let _ = on_filter;
    let state = RwSignal::new(DataTableState::default());

    // Initialize state with props
    if let Some(data) = data.get_untracked() {
        state.update(|s| s.data = data);
    }
    if let Some(columns) = columns.get_untracked() {
        state.update(|s| s.columns = columns);
    }
    if let Some(loading) = loading.get_untracked() {
        state.update(|s| s.loading = loading);
    }
    if let Some(error) = error.get_untracked() {
        state.update(|s| s.error = error);
    }

    // Computed values
    let filtered_data = Signal::derive(move || {
        let state = state.get();
        let mut filtered = state.data.clone();

        // Apply filters
        for filter in &state.filters {
            if filter.active {
                filtered.retain(|row| match filter.column_key.as_str() {
                    "name" => match filter.operator {
                        FilterOperator::Contains => row.name.contains(&filter.value),
                        FilterOperator::Equals => row.name == filter.value,
                        FilterOperator::StartsWith => row.name.starts_with(&filter.value),
                        FilterOperator::EndsWith => row.name.ends_with(&filter.value),
                        _ => true,
                    },
                    "email" => match filter.operator {
                        FilterOperator::Contains => row.email.contains(&filter.value),
                        FilterOperator::Equals => row.email == filter.value,
                        _ => true,
                    },
                    "age" => {
                        if let Ok(age) = filter.value.parse::<i32>() {
                            match filter.operator {
                                FilterOperator::Equals => row.age == age,
                                FilterOperator::GreaterThan => row.age > age,
                                FilterOperator::LessThan => row.age < age,
                                _ => true,
                            }
                        } else {
                            true
                        }
                    }
                    _ => true,
                });
            }
        }

        // Apply sorting
        if state.sort_config.active {
            filtered.sort_by(|a, b| match state.sort_config.column_key.as_str() {
                "name" => match state.sort_config.direction {
                    SortDirection::Ascending => a.name.cmp(&b.name),
                    SortDirection::Descending => b.name.cmp(&a.name),
                    _ => std::cmp::Ordering::Equal,
                },
                "email" => match state.sort_config.direction {
                    SortDirection::Ascending => a.email.cmp(&b.email),
                    SortDirection::Descending => b.email.cmp(&a.email),
                    _ => std::cmp::Ordering::Equal,
                },
                "age" => match state.sort_config.direction {
                    SortDirection::Ascending => a.age.cmp(&b.age),
                    SortDirection::Descending => b.age.cmp(&a.age),
                    _ => std::cmp::Ordering::Equal,
                },
                _ => std::cmp::Ordering::Equal,
            });
        }

        filtered
    });

    let paginated_data = Signal::derive(move || {
        let state = state.get();
        let filtered = filtered_data.get();
        let start = (state.pagination.current_page - 1) * state.pagination.page_size;

        filtered
            .into_iter()
            .skip(start as usize)
            .take(state.pagination.page_size as usize)
            .collect::<Vec<_>>()
    });

    // Event handlers
    let handle_sort = move |column_key: String| {
        let current_sort = state.get().sort_config.clone();
        let new_direction = if current_sort.column_key == column_key {
            match current_sort.direction {
                SortDirection::None => SortDirection::Ascending,
                SortDirection::Ascending => SortDirection::Descending,
                SortDirection::Descending => SortDirection::None,
            }
        } else {
            SortDirection::Ascending
        };

        let new_sort = SortConfig {
            column_key: column_key.clone(),
            direction: new_direction,
            active: new_direction != SortDirection::None,
        };

        state.update(|s| s.sort_config = new_sort.clone());

        if let Some(callback) = &on_sort {
            callback.run(new_sort);
        }
    };

    let handle_row_click = move |row: DataRow| {
        if let Some(callback) = &on_row_click {
            callback.run(row);
        }
    };

    let handle_row_select = move |row_id: i32| {
        let mut selected = state.get().selection.selected_rows.clone();
        if selected.contains(&row_id) {
            selected.retain(|&id| id != row_id);
        } else {
            selected.push(row_id);
        }

        state.update(|s| s.selection.selected_rows = selected.clone());

        if let Some(callback) = &on_row_select {
            callback.run(selected);
        }
    };

    view! {
        <div
            class=move || format!("data-table {}", class.get().unwrap_or_default())
            id=move || id.get()
            style=move || style.get()
        >
            // Table header
            <div class="data-table-header">
                <div class="data-table-title">
                    "Data Table"
                </div>
                <div class="data-table-actions">
                    <button
                        class="data-table-export-btn"
                        on:click=move |_| {
                            if let Some(callback) = &on_export {
                                callback.run(ExportFormat::Csv);
                            }
                        }
                    >
                        "Export CSV"
                    </button>
                </div>
            </div>

            // Table content
            <div class="data-table-content">
                {move || {
                    let current = state.get();
                    if current.loading {
                        view! { <div class="data-table-loading">"Loading..."</div> }.into_any()
                    } else if let Some(error) = current.error.clone() {
                        view! { <div class="data-table-error">{error}</div> }.into_any()
                    } else {
                        view! {
                            <table class="data-table-table">
                                <thead>
                                    <tr>
                                        {current.columns.into_iter().map(|column| {
                                            let column_key = column.key.clone();
                                            let sort_key = column.key.clone();
                                            let sortable = column.sortable;
                                            view! {
                                                <th
                                                    class=format!("data-table-header-cell {}", if sortable { "sortable" } else { "" })
                                                    on:click=move |_| if sortable { handle_sort(column_key.clone()) }
                                                >
                                                    {column.title.clone()}
                                                    {move || {
                                                        let st = state.get();
                                                        if st.sort_config.column_key == sort_key {
                                                            match st.sort_config.direction {
                                                                SortDirection::Ascending => " ↑",
                                                                SortDirection::Descending => " ↓",
                                                                _ => "",
                                                            }
                                                        } else {
                                                            ""
                                                        }
                                                    }}
                                                </th>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                </thead>
                                <tbody>
                                    {paginated_data.get().into_iter().map(|row| {
                                        let row_clone = row.clone();
                                        view! {
                                            <tr
                                                class="data-table-row"
                                                on:click=move |_| {
                                                    handle_row_select(row_clone.id);
                                                    handle_row_click(row_clone.clone());
                                                }
                                            >
                                                <td class="data-table-cell">{row.name.clone()}</td>
                                                <td class="data-table-cell">{row.email.clone()}</td>
                                                <td class="data-table-cell">{row.age}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        }.into_any()
                    }
                }}
            </div>

            // Table footer with pagination
            <div class="data-table-footer">
                <div class="data-table-pagination">
                    <button
                        class="data-table-pagination-btn"
                        disabled=move || state.get().pagination.current_page <= 1
                        on:click=move |_| {
                            state.update(|s| {
                                if s.pagination.current_page > 1 {
                                    s.pagination.current_page -= 1;
                                }
                            });
                        }
                    >
                        "Previous"
                    </button>
                    <span class="data-table-pagination-info">
                        {move || {
                            let state = state.get();
                            format!("Page {} of {}", state.pagination.current_page, state.pagination.total_pages)
                        }}
                    </span>
                    <button
                        class="data-table-pagination-btn"
                        disabled=move || state.get().pagination.current_page >= state.get().pagination.total_pages
                        on:click=move |_| {
                            state.update(|s| {
                                if s.pagination.current_page < s.pagination.total_pages {
                                    s.pagination.current_page += 1;
                                }
                            });
                        }
                    >
                        "Next"
                    </button>
                </div>
            </div>

            {children.map(|c| c())}
        </div>
    }
}
