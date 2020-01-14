use lib '.';
use t::Helper;

my $tt = t::Helper->tt;
ok $tt->can("command_$_"), "tt $_" for qw(export log start stop status register);

done_testing;
