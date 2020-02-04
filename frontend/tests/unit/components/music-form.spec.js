import { shallowMount } from '@vue/test-utils';
import MusicForm from '@/components/music-form.vue';
import moxios from 'moxios';

describe('music-form.vue', () => {

    beforeEach( () => {
        // import and pass your custom axios instance to this method
        moxios.install();
    });
    afterEach( () => {
        // import and pass your custom axios instance to this method
        moxios.uninstall();
    });

    it('renders', (done) => {
        const wrapper = shallowMount(MusicForm,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "ARTIST",
                    "disc": "DISC",
                },
            }
        });
      expect(wrapper.html()).toBe(`<div class=\"edit-form\"><input class=\"music-helper\"> <edit-field-stub field=\"ARTIST\" label=\"artist\"></edit-field-stub> <edit-field-stub field=\"DISC\" label=\"disc\"></edit-field-stub> <button class=\"save\">Save</button> <button class=\"remove\">Delete</button></div>`);
      done();
    });

    it('helps on paste', async (done) => {
        const item = {
            "id": 1,
            "artist": "ARTIST",
            "disc": "DISC",
        };
        const wrapper = shallowMount(MusicForm,{
            propsData: {
                'item' : item,
            }
        });
        const helper = wrapper.find('.music-helper');
        helper.setValue( "ARTIST2 - DISC2" );
        helper.trigger('change');
        await wrapper.vm.$nextTick()
        expect(wrapper.props().item).toEqual({
            "id": 1,
            "artist": "ARTIST2",
            "disc": "DISC2",
        });
        done();
    });

    it('updates', async (done) => {
        const item = {
            "id": 1,
            "artist": "ARTIST",
            "disc": "DISC",
        };
        const wrapper = shallowMount(MusicForm,{
            propsData: {
                'item' : item,
            }
        });
        const save_button = wrapper.find('.save');
        save_button.trigger('click');

        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/music/1');
            expect(request.config.method).toEqual('put');
            const requestBody = JSON.parse(request.config.data);
            expect(requestBody).toEqual(item);
            request.respondWith({
              status: 200,
            }).then(() => {
                expect(wrapper.emitted()["music-saved"]).toBeTruthy();
                expect(wrapper.emitted()["music-saved"]).toEqual([[item]]);
                done();
            });
        });

    });

    it('creates', async (done) => {
        const item = {
            "artist": "ARTIST",
            "disc": "DISC",
        };
        const wrapper = shallowMount(MusicForm,{
            propsData: {
                'item' : item,
            }
        });
        const save_button = wrapper.find('.save');
        save_button.trigger('click');

        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/music/');
            expect(request.config.method).toEqual('post');
            const requestBody = JSON.parse(request.config.data);
            expect(requestBody).toEqual(item);
            request.respondWith({
              status: 200,
            }).then(() => {
                expect(wrapper.emitted()["music-saved"]).toBeTruthy();
                expect(wrapper.emitted()["music-saved"]).toEqual([[item]]);
                done();
            });
        });

    });

    it('deletes', async (done) => {
        const item = {
            "id": 1,
            "artist": "ARTIST",
            "disc": "DISC",
        };
        const wrapper = shallowMount(MusicForm,{
            propsData: {
                'item' : item,
            }
        });
        const save_button = wrapper.find('.remove');
        save_button.trigger('click');

        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/music/1');
            expect(request.config.method).toEqual('delete');
            request.respondWith({
              status: 200,
            }).then(() => {
                expect(wrapper.emitted()["music-deleted"]).toBeTruthy();
                expect(wrapper.emitted()["music-deleted"]).toEqual([[item]]);
                done();
            });
        });

    });

});
