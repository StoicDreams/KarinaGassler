// Settings for the shared checks in site.spec.mjs, which is the same file in every site.
export default {
    // Shared checks this site is known to fail, as { 'test title': 'Known bug: reason' }.
    // The test still runs and flags itself once the bug is fixed. Remove the entry when fixing the bug.
    knownBugs: {
        'links between pages go to pages that exist': 'Known bug: contact.md shows /img/58.avif and /img/13.avif, which are not in root_files/img',
    },
};
