use 5.016;
use strict;
use warnings;
use utf8;
use ExtUtils::MakeMaker;

my $GITHUB_URL = 'https://github.com/jhthorsen/app-tt';

my %WriteMakefileArgs = (
  AUTHOR        => 'Jan Henning Thorsen <jhthorsen@cpan.org>',
  LICENSE       => 'artistic_2',
  NAME          => 'App::tt',
  ABSTRACT_FROM => 'lib/App/tt.pm',
  VERSION_FROM  => 'lib/App/tt.pm',
  EXE_FILES     => [],
  TEST_REQUIRES => {'Test::More' => '0.88'},
  PREREQ_PM     =>
    {'Getopt::App' => '0.13', 'JSON::PP' => '2.20', 'List::Util' => '1.45', 'perl' => '5.020001'},
  META_MERGE => {
    'dynamic_config' => 0,
    'meta-spec'      => {version   => 2},
    'no_index'       => {directory => [qw(examples t)]},
    'prereqs'        => {runtime   => {requires => {perl => '5.016'}}},
    'resources'      => {
      bugtracker => {web => "$GITHUB_URL/issues"},
      homepage   => $GITHUB_URL,
      license    => ['http://www.opensource.org/licenses/artistic-license-2.0'],
      repository => {type => 'git', url => "$GITHUB_URL.git", web => $GITHUB_URL},
      x_IRC => {url => 'irc://irc.libera.chat/#convos', web => 'https://web.libera.chat/#convos'},
    },
    'x_contributors' => [],
  },
  test => {TESTS => (-e 'META.yml' ? 't/*.t' : 't/*.t xt/*.t')},
);

unless (eval { ExtUtils::MakeMaker->VERSION('6.63_03') }) {
  my $test_requires = delete $WriteMakefileArgs{TEST_REQUIRES};
  @{$WriteMakefileArgs{PREREQ_PM}}{keys %$test_requires} = values %$test_requires;
}

WriteMakefile(%WriteMakefileArgs);
