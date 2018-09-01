use commands::Command;

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, _args: Vec<String>) -> String {

        // create tree from index
        //   format:
        //     100644 blob 03554fdfc16c48b1f9e3b47c772b94310f52af23	Procfile
        //     100755 blob 48c65185f28c9643bb1add7a5112407fbe50a407	main.rb
        //     040000 tree 65461b2e7e1dd3500f27b0b856ed5661ec461a34	migrations
        //     index:
        //       mode
        //       type
        //       sha1
        //       filename
        //   calculate_sha1
        //   deflate_contents
        //   store_deflated_contents
        //
        // create commit from tree
        //   format:
        //     tree ebc122424667a98c51bf2bb816367b39f1ae1ca4
        //     parent ac5198984abfec497f4f4f54b0d353fec46b3aa3
        //     <blankline>
        //     the message goes here
        //   calculate_sha1
        //   deflate_contents
        //   store_deflated_contents

        // committing clears the index

        // return sha1

        //////////////////////////////
        //////////////////////////////
        // implementation goes here //
        //////////////////////////////
        //////////////////////////////

        "Committed...".to_string() // SHA-1
    }
}