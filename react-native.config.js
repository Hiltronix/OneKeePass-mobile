// Match Metro's JS camera stub with native dependency exclusion.
const fossBuild = process.env.OKP_FOSS_BUILD === '1';

module.exports = {
  dependencies: fossBuild
    ? {
        'react-native-vision-camera': {
          platforms: {android: null, ios: null},
        },
      }
    : {},
};
