class BindingLibraryLoader {
    static class Target {
        final String directory;
        final String name;
        final String extension;

        Target(String directory, String name, String extension) {
            this.directory = directory;
            this.name = name;
            this.extension = extension;
        }

        String path() {
            return "/" + this.directory + "/" + this.name + "." + this.extension;
        }
    }

    void loadLibrary(Target[] targets) {

    }

    static void loadTargets(Target[] targets) throws Exception {
        if (targets.length == 0) {
            throw new Exception("No targets were specified");
        }
        Exception lastException = null;
        for (Target target : targets) {
            try {
                loadTargetLibrary(target);
                return;
            } catch (Exception ex) {
                lastException = ex;
            }
        }
        throw lastException;
    }

    private static void loadTargetLibrary(Target target) throws Exception {
        final String path = target.path();
        java.io.InputStream stream = NativeFunctions.class.getResourceAsStream(path);
        if (stream == null) {
            throw new Exception("Resource not found: " + path);
        }
        java.nio.file.Path tempFilePath = java.nio.file.Files.createTempFile(target.name, "." + target.extension);
        tempFilePath.toFile().deleteOnExit();
        java.nio.file.Files.copy(stream, tempFilePath, java.nio.file.StandardCopyOption.REPLACE_EXISTING);
        System.load(tempFilePath.toAbsolutePath().toString());
    }
}
