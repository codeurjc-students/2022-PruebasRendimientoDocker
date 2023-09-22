module.exports = {
    canNextPage: canNextPage, 
}

function canNextPage(context, next) {
    const page = context.vars

    // This means that could't find more items to reach the limit, so there are no more pages
    const exist_next_page = page.limit > page.size

    return next(exist_next_page)
}
